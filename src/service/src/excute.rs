
use std::default::Default;
use crate::sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbConn, DbErr};

use morioka_entity::character;

pub struct Excute;
impl Excute{
    pub async fn make_new_character(
        db: &DatabaseConnection,
        acc_id: i32,
        ch_name: Option<String>,
        ch_nick_name: Option<String>,
    ) -> Result<Option<String>, DbErr> {
        let new_character = character::ActiveModel {
            account_id: Set(Some(acc_id).to_owned()),
            character_name: Set(ch_name),
            character_nickname: Set(ch_nick_name),
            is_valid:Set(Some(1)),
            ..Default::default()
        };
        let _ = new_character.insert(db).await?;
        Ok(Some("make new character OK".to_string()))
    }
}