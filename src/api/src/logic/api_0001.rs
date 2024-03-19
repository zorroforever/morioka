use std::env;
use std::sync::Arc;

use actix_web::{Error, get, HttpResponse, web};
// use actix_web::web::Buf;
use serde_json::json;
use tokio::sync::RwLock as AsyncRwLock;
use web::Data;

use morioka_service::Query;

use crate::common;
use crate::util::crypto_util;

#[get("/fetch_token/{mix_id}")]
pub async fn fetch_token(
    data: Data<Arc<AsyncRwLock<common::AppState>>>,
    mix_id: web::Path<String>,
) -> actix_web::Result<HttpResponse, Error> {
    let app_status = data.get_ref().read().await;
    let app_status_ref = &*app_status;
    // let app_status = data.read().await;
    // let app_status_ref = &*app_status;
    // let app_status = app_status_ref.get_ref();
    let conn = &app_status_ref.db_conn;
    let id = &mix_id;
    let key = env::var("ENCRYPTION_KEY").expect("ENCRYPTION_KEY is not set in .env file");
    println!("aes key: {key:?}");
    let source_str = crypto_util::aes_decrypt(id,&key.into_bytes()).await;
    let token = Query::fetch_token_by_mix(conn,source_str).await.unwrap_or(Option::Some("".to_string()));
    if let Some(a) = token {
        Ok(HttpResponse::Ok().json(json!({"token": a}).clone()))
    } else {
        Ok(HttpResponse::Ok().json("{}"))
    }
}