use morioka_service::sea_orm::DatabaseConnection;

pub async fn handle(
    db: &DatabaseConnection,
    option: Option<serde_json::Value>,
) -> String {
    let json_str: serde_json::Value = option.expect("wrong param!");
    let direction = json_str["direction"].as_str().unwrap().to_string();
    let account_id = json_str["aid"].as_i64().unwrap();
    let character_id = json_str["cid"].as_i64().unwrap();

    // TODO: implement logic
    "".to_string()

}