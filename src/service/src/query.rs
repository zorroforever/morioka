use sea_orm::*;

use ::morioka_entity::{token_lake, token_lake::Entity as TokenLake};
use ::morioka_entity::{account, account::Entity as Account};

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
}
