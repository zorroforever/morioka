
use std::default::Default;
use crate::sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbConn, DbErr};
use sea_orm::prelude::{DateTime, Decimal};

use morioka_entity::{character, character_propety};

pub struct Excute;
impl Excute{
    pub async fn make_new_character(
        db: &DatabaseConnection,
        acc_id: i32,
        ch_name: Option<String>,
        ch_nick_name: Option<String>,
        bazi_year_tiangan: i32,
        bazi_year_dizhi: i32,
        bazi_month_tiangan: i32,
        bazi_month_dizhi: i32,
        bazi_day_tiangan: i32,
        bazi_day_dizhi: i32,
        bazi_hour_tiangan: i32,
        bazi_hour_dizhi: i32,
        create_time: DateTime,
    ) -> Result<Option<String>, DbErr> {
        let new_character = character::ActiveModel {
            account_id: Set(Some(acc_id).to_owned()),
            character_name: Set(ch_name),
            character_nickname: Set(ch_nick_name),
            is_valid:Set(Some(1)),
            create_time:Set(Some(create_time)),
            ..Default::default()
        };
        let md = new_character.insert(db).await?;
        let new_character_id = md.id;

       let new_character_propety = character_propety::ActiveModel{
           character_id: Set(Some(new_character_id)),
           health_point: Set(Some(Decimal::new(10000,2))),
           mana_point: Set(Some(Decimal::new(1000,2))),
           bazi_year_tiangan: Set(Some(bazi_year_tiangan)),
           bazi_year_dizhi: Set(Some(bazi_year_dizhi)),
           bazi_month_tiangan: Set(Some(bazi_month_tiangan)),
           bazi_month_dizhi: Set(Some(bazi_month_dizhi)),
           bazi_day_tiangan: Set(Some(bazi_day_tiangan)),
           bazi_day_dizhi: Set(Some(bazi_day_dizhi)),
           bazi_hour_tiangan: Set(Some(bazi_hour_tiangan)),
           bazi_hour_dizhi: Set(Some(bazi_hour_dizhi)),
           create_time: Set(Some(create_time)),
           ..Default::default()
       };
        let md2= new_character_propety.insert(db).await?;
        Ok(Some(format!("make new character OK,{:?}",md2)))
    }
}