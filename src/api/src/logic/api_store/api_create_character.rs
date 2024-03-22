use serde_json::json;
use std::sync::Arc;
use actix_web::web;
use morioka_service::Excute;
use crate::common;
use tokio::sync::RwLock as AsyncRwLock;
use morioka_service::sea_orm::DatabaseConnection;

pub async fn handle(
    db: &DatabaseConnection,
    option: Option<serde_json::Value>,
) -> String {
    let json_str: serde_json::Value = option.expect("wrong param!");
    let _ = Excute::make_new_character(
        db,
                               1,
        Some(json_str["ch_name"].as_str().unwrap_or_default().to_string()),
        Some(json_str["ch_nickname"].as_str().unwrap_or_default().to_string()),
    ).await;
    "{}".to_string()
}