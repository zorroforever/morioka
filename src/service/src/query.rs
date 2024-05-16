use sea_orm::*;

use ::morioka_entity::{token_lake, token_lake::Entity as TokenLake};
use ::morioka_entity::{account, account::Entity as Account};
use ::morioka_entity::{character, character::Entity as Character};
use ::morioka_entity::{map, map::Entity as Map};
use ::morioka_entity::{map_detail, map_detail::Entity as Map_detail};

pub struct Query;

impl Query {
    pub async fn fetch_token_by_mix(
        db: &DbConn,
        mix_id: String,
    ) -> Result<Option<String>, DbErr> {
        let token_data: Option<token_lake::Model> = TokenLake::find()
            .filter(token_lake::Column::MixCode.eq(mix_id)).one(db).await?;
        if let Some(_v) = token_data {
            let token = _v.token;
            return Ok(token);
        } else {
            Ok(Option::Some("udhsyw".to_string()))
        }
    }

    pub async fn check_account(
        db: &DbConn,
        mix_code: &str,
    ) -> Result<bool, DbErr> {
        let cnt = Account::find()
            .filter(account::Column::MixCode.eq(mix_code))
            .filter(account::Column::IsValid.eq(true))
            .count(db).await?;
        if cnt > 0 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn get_account_by_mix_code(
        db: &DbConn,
        mix_code: &str,
    ) -> Result<i32, DbErr> {
        let acc_model= Account::find()
            .filter(account::Column::MixCode.eq(mix_code))
            .filter(account::Column::IsValid.eq(true))
            .one(db).await?;
        let res = acc_model.unwrap();
       Ok(res.id)
    }

    pub async fn list_characters_by_account_id(
        db: &DbConn,
        aid: i32,
    ) -> Result<Vec<String>, DbErr> {
        let ch_list = Character::find()
            .filter(character::Column::AccountId.eq(aid))
            .filter(character::Column::IsValid.eq(1))
            .all(db).await?;
        let res: Vec<String> = ch_list.iter().map(|md| format!("{:?}", md)).collect();
        Ok(res)

    }
    pub async fn load_map_by_mid(
        db: &DbConn,
        mid: i32,
    ) -> Result<map::Model, DbErr> {
        let map = Map::find()
            .filter(map::Column::Id.eq(mid))
            .one(db).await?;
        Ok(map.unwrap())
    }

    pub async fn load_map_detail_by_mid(
        db: &DbConn,
        mid: i32,
    ) -> Result<Vec<map_detail::Model>, DbErr> {
            let map_detail = Map_detail::find()
            .filter(map_detail::Column::Id.eq(mid))
            .all(db).await?;
        Ok(map_detail)
    }
}
