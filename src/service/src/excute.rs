use std::default::Default;

use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait};
use sea_orm::prelude::{DateTime, Decimal};

use morioka_entity::{character, character_propety};
use ::morioka_entity::{character_status, character_status::Entity as Character_status};
use crate::sea_orm::ActiveValue::Set;
use sea_orm::QueryFilter;
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
    ) -> Result<i32, DbErr> {
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
        Ok(new_character_id)
    }

    pub async fn update_character_position_on_map(
        db: &DatabaseConnection,
        map_id: i32,
        character_id: i32,
        position_x: i32,
        position_y: i32,
        position_z: i32,
    )-> Result<(), DbErr> {
        // find character data on character_status table by character_id
        let character_status = Character_status::find()
            .filter(character_status::Column::CharacterId.eq(character_id))
            .one(db)
            .await?;
        if character_status.is_some() {
            let _ = character_status::ActiveModel {
                map_id: Set(Some(map_id)),
                character_id: Set(Some(character_id)),
                x: Set(Some(position_x)),
                y: Set(Some(position_y)),
                z: Set(Some(position_z)),
                ..Default::default()
            }.update(db).await?;
        } else {
             let new_character_status = character_status::ActiveModel {
                map_id: Set(Some(map_id)),
                character_id: Set(Some(character_id)),
                x: Set(Some(position_x)),
                y: Set(Some(position_y)),
                z: Set(Some(position_z)),
                ..Default::default()
            };
            let _ = new_character_status.insert(db).await?;
        }

    Ok(())
    }
}