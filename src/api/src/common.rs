use redis::Connection;
use serde::{Deserialize, Serialize};

use morioka_service::sea_orm::DatabaseConnection;

pub struct AppState {
    pub(crate) db_conn: DatabaseConnection,
    pub(crate) redis_conn: Connection,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct MoriokaParams {
    token: Option<String>,
    udid: Option<String>,
    data:Option<String>,
}