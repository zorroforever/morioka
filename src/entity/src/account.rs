//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use serde::{Serialize,Deserialize};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq,Serialize, Deserialize)]
#[sea_orm(table_name = "account")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub account_name: String,
    pub mix_code: Option<String>,
    pub is_valid: Option<i8>,
    pub register_time: Option<DateTime>,
    pub update_time: Option<DateTime>,
    pub create_time: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
