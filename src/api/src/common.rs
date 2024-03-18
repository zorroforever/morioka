use serde::{Deserialize, Serialize};
use morioka_service::sea_orm::DatabaseConnection;

#[derive(Debug, Clone)]
pub struct AppState {
    pub(crate) conn: DatabaseConnection,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct MoriokaParams {
    token: Option<String>,
    udid: Option<String>,
    data:Option<String>,
}