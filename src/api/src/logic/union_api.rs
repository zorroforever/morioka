use std::sync::Arc;
use actix_web::{Error, HttpRequest, HttpResponse, post, web};
use actix_web::web::Data;
use serde_json::from_slice;
use crate::{common, util};
use tokio::sync::RwLock as AsyncRwLock;
use redis::{Commands, Connection, RedisResult};
#[post("/v1/api/{token}")]
pub async fn api(
    token: web::Path<String>,
    app_data: web::Data<Arc<AsyncRwLock<common::AppState>>>,
    body: web::Bytes,
    _req: HttpRequest,
) -> actix_web::Result<HttpResponse, Error> {
    println!("enter to apitoken={}",&token);
    let mut app_status = app_data.write().await;
    let redis_conn = &app_status.redis_conn;
    let token_string: String = token.into_inner();
    let token_is_valid  =redis_conn.check_token_validity(&token_string).await.unwrap_or(false);

    if !token_is_valid {
        println!("wrong token");
        return Ok::<HttpResponse, Error>(HttpResponse::Forbidden().json("{}"));
    }

    let item = from_slice::<common::MoriokaApiParams>(&body)?;
    println!("model: {item:?}");
    if let Some(api_key) = item.get_api_key() {
        match api_key.as_str() {
            "api_check_token"=> {

            }
            _ => {
                println!("key is invalid!");
            }
        }
    } else {
        println!("api key is empty!");
    }

    Ok(HttpResponse::Ok().json("{}"))
}