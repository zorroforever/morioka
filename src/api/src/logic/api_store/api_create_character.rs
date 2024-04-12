use serde_json::json;
use std::sync::Arc;
use actix_web::web;
use morioka_service::Excute;
use crate::common;
use tokio::sync::RwLock as AsyncRwLock;
use morioka_service::sea_orm::DatabaseConnection;
use crate::util::calculate_bazi;

pub async fn handle(
    db: &DatabaseConnection,
    option: Option<serde_json::Value>,
) -> String {
    let json_str: serde_json::Value = option.expect("wrong param!");
    let bazi = calculate_bazi().await.unwrap();
    let res = Excute::make_new_character(
        db,
        json_str["aid"].as_i64().map(|v| v as i32).unwrap_or_default().to_owned(),
        Some(json_str["ch_name"].as_str().unwrap_or_default().to_string()),
        Some(json_str["ch_nickname"].as_str().unwrap_or_default().to_string()),
        bazi.year_tiangan,
        bazi.year_dizhi,
        bazi.month_tiangan,
        bazi.month_dizhi,
        bazi.day_tiangan,
        bazi.day_dizhi,
        bazi.hour_tiangan,
        bazi.hour_dizhi,
        chrono::Local::now().naive_local(),
    ).await;

    res.unwrap().unwrap()
}