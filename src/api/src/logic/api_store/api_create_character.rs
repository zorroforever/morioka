use morioka_service::Excute;
use morioka_service::sea_orm::DatabaseConnection;
use serde_json;

use crate::util::{calculate_bazi, map_util, MoriokaPosition};

pub async fn handle(
    db: &DatabaseConnection,
    option: Option<serde_json::Value>,
) -> String {
    let json_str: serde_json::Value = option.expect("wrong param!");
    let bazi = calculate_bazi().await.unwrap();
    // init character
    let character_id = Excute::make_new_character(
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
    let ch_id = character_id.unwrap();
    // init map ,for id 1
    let current_map = map_util::load_map(db,1).await;
    // set (0,0,0) as character position on current map
    let postion = MoriokaPosition { x: 0, y: 0, z: 0 };
    let _ = map_util::set_character_position_on_map(db, 1,ch_id, postion).await;
    "Ok".to_string()
}

