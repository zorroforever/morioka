use serde::{Deserialize, Serialize};

use morioka_service::sea_orm::DatabaseConnection;

use crate::util::redis_util::MoriokaRedis;

pub struct AppState {
    pub(crate) db_conn: DatabaseConnection,
    pub(crate) redis_conn: MoriokaRedis,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MoriokaParams {
    token: Option<String>,
    udid: Option<String>,
    data: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MoriokaApiParams {
    api_key: Option<String>,
    data: Option<serde_json::Value>,
}

impl MoriokaApiParams {
    pub fn get_api_key(&self) -> Option<String> {
        self.api_key.clone().take()
    }
    pub fn get_data(&self) -> Option<serde_json::Value> {
        self.data.clone().take()
    }
}

