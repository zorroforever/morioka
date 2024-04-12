use morioka_service::Query;
use morioka_service::sea_orm::DatabaseConnection;

pub async fn handle(
    db: &DatabaseConnection,
    option: Option<serde_json::Value>,
) -> Vec<String> {
    let json_str: serde_json::Value = option.expect("wrong param!");
    let res = Query::list_characters_by_account_id(
        db,
        json_str["aid"].as_i64().map(|v| v as i32).unwrap_or_default().to_owned(),
    ).await;

    res.unwrap()
}