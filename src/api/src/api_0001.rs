use crate::crypto_util;
use std::env;
use actix_web::{Error, get, HttpResponse, web};
use serde_json::json;
use web::Data;
use morioka_service::Query;

use crate::common;


#[get("/fetch_token/{mix_id}")]
async fn fetch_token(
    data: Data<common::AppState>,
    mix_id: web::Path<String>,
) -> actix_web::Result<HttpResponse, Error> {
    let conn = &data.conn;
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