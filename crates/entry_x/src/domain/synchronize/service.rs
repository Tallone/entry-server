use sea_orm::{
  sea_query::{Alias, Expr, OnConflict}, ColumnTrait, ConnectionTrait, DatabaseConnection, EntityTrait, Iden, QueryFilter, Set, Statement
};
use uuid::Uuid;

use crate::domain::{
  entity::synchronize::{self, Column, Entity, Model},
  Result,
};

pub async fn get(
  conn: &sea_orm::DatabaseConnection,
  user_id: Uuid,
) -> std::result::Result<Option<synchronize::Model>, sea_orm::DbErr> {
  let resp = Entity::find().filter(Column::UserId.eq(user_id)).one(conn).await?;
  Ok(resp)
}

pub async fn save<'a, C: ConnectionTrait>(conn: &C, user_id: Uuid, content: String) -> Result<Model> {
  let act_mod = synchronize::ActiveModel {
    user_id: Set(user_id),
    version: Set(1),
    content: Set(content.clone()),
    ..Default::default()
  };

  let ret = Entity::insert(act_mod)
    .on_conflict(
      OnConflict::column(Column::UserId)
        .value(Column::Content, Expr::val(content))
        .value(Column::Version, Expr::custom_keyword(Alias::new(format!("{}.{}", Column::Version.entity_name().to_string(), Column::Version.to_string()))).add(1))
        .to_owned(),
    )
    .exec_with_returning(conn)
    .await?;
  Ok(ret)
}

pub async fn clear(conn: &DatabaseConnection, user_id: Uuid) -> Result<bool> {
  let ret = synchronize::Entity::update_many()
    .set(synchronize::ActiveModel {
      content: Set(String::default()),
      ..Default::default()
    })
    .col_expr(Column::Version, Expr::col(Column::Version).add(1))
    .filter(Column::UserId.eq(user_id))
    .exec(conn)
    .await?;

  Ok(ret.rows_affected > 0)
}

#[cfg(test)]
mod tests {
  use super::*;
  use dotenvy::dotenv;
  use log::info;
  use sea_orm::TransactionTrait;
use serde_json::json;
  use uuid::Uuid;

  use crate::{conf::ApplicationConf, db::DB};

  async fn init() -> DB {
    dotenv().expect(".env file not found");
    let _ = env_logger::builder()
      .filter_level(log::LevelFilter::Info)
      .is_test(true)
      .try_init();

    let conf = ApplicationConf::from_env();
    DB::new(&conf).await.unwrap()
  }

  #[tokio::test]
  async fn test_save() {
    let db = init().await;
    let user_id = Uuid::new_v4();
    let content = json!(
      {
        "note": "This is user's note"
      }
    );

    let txn = db.conn.begin().await.unwrap();
    for i in 0..3 {
      let model = save(&txn, user_id, content.to_string()).await.unwrap();
      assert_eq!(model.version, i + 1);
    }
    txn.rollback().await.unwrap();
  }
}
