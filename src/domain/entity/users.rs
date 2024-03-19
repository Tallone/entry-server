//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  pub id: Uuid,
  #[sea_orm(column_type = "Text", unique)]
  pub email: String,
  #[sea_orm(column_type = "Text")]
  #[serde(skip_deserializing)]
  pub password: String,
  #[sea_orm(column_type = "Text")]
  #[serde(skip_deserializing)]
  pub hash: String,
  pub status: i16,
  pub created_at: TimeDateTimeWithTimeZone,
  pub updated_at: TimeDateTimeWithTimeZone,
  pub name: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
