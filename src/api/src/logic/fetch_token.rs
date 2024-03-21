use uuid::Uuid;
use std::env;
use std::sync::Arc;

use actix_web::{Error, get, HttpResponse, web};
// use actix_web::web::Buf;
use serde_json::json;
use tokio::sync::RwLock as AsyncRwLock;
use web::Data;
use morioka_service::Query;
use crate::{common, util};
use crate::util::crypto_util;

#[get("/fetch_token/{mix_id}")]
pub async fn fetch_token(
    data: Data<Arc<AsyncRwLock<common::AppState>>>,
    mix_id: web::Path<String>,
) -> actix_web::Result<HttpResponse, Error> {
    let app_status = data.get_ref().write().await;
    let conn = &app_status.db_conn;
    let redis_conn =  &app_status.redis_conn;
    let id = &mix_id;
    // get source str
    let key = env::var("ENCRYPTION_KEY").expect("ENCRYPTION_KEY is not set in .env file");
    let source_str = crypto_util::aes_decrypt(id,&key.into_bytes()).await;
    let source_md5_str = crypto_util::md5_encrypt(&source_str).await;
    // get token on redis
    if let Ok(_v) =redis_conn.check_token_validity(&source_md5_str).await {
        if let Ok(_v1) =redis_conn.get_val_by_key(&source_md5_str).await {
            return Ok(HttpResponse::Ok().json(json!({"token": _v1}).clone()));
        }
    }
    // try to login account
    let nice_to_login = Query::check_account(conn,&source_md5_str).await.unwrap_or(false);
    if nice_to_login {
        let uuid = Uuid::new_v4();
        redis_conn.set_token_with_expiry(
            &uuid.to_string(),
            &source_md5_str,
            3600)
            .await;
        Ok(HttpResponse::Ok().json(json!({"token": uuid.to_string()}).clone()))
    } else {
        Ok(HttpResponse::Unauthorized().json("{}"))
    }
}