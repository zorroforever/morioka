use morioka_service::sea_orm::DatabaseConnection;

pub async fn handle(
    db: &DatabaseConnection,
    option: Option<serde_json::Value>,
) -> String {
    let json_str: serde_json::Value = option.expect("wrong param!");

    "".to_string()

}