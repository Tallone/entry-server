use std::time::Duration;

use anyhow::anyhow;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use util::{cache::redis, KeysInterface};
use uuid::Uuid;

use crate::{
  domain::{entity::users, Result},
  error::AppError,
};

use super::model::GetReq;

const DEFAULT_CACHE_DURATION: Duration = Duration::from_secs(2 * 30 * 60);

pub(crate) struct Mutation;
pub(crate) struct Query;

impl Mutation {
  pub async fn create(db: &DatabaseConnection, model: users::ActiveModel) -> Result<users::Model> {
    let ret = model.insert(db).await?;
    Ok(ret)
  }
}

impl Query {
  pub async fn get(db: &DatabaseConnection, opt: GetReq) -> Result<users::Model> {
    let query = users::Entity::find();
    let query = match opt {
      GetReq::Id(id) => query.filter(users::Column::Id.eq(id)),
      GetReq::Email(v) => query.filter(users::Column::Email.eq(v)),
    };
    let resp = query.one(db).await?;
    resp.ok_or(AppError::RecordNotFound)
  }
}

pub async fn get_user(cache_key: &str, id: &str, db: &DatabaseConnection) -> Result<Option<users::Model>> {
  let redis = util::cache::redis().await;
  let count: usize = redis.exists(cache_key).await?;
  if count > 0 {
    let json: String = redis.get(cache_key).await?;
    let u: users::Model = serde_json::from_str(&json).map_err(|_| anyhow!("Got an invlid user json from redis"))?;
    return Ok(Some(u));
  }

  let uid = Uuid::parse_str(id).map_err(|_| anyhow!("Not a valid uuid: {}", id))?;
  if let Some(u) = users::Entity::find_by_id(uid).one(db).await? {
    let data = serde_json::to_string(&u).unwrap();
    redis
      .set(
        cache_key,
        &data,
        Some(util::Expiration::EX(DEFAULT_CACHE_DURATION.as_secs() as i64)),
        None,
        false,
      )
      .await?;

    return Ok(Some(u));
  }
  Ok(None)
}

#[cfg(test)]
mod tests {
  use super::*;
  use dotenvy::dotenv;
  use log::info;

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
  async fn test_get_from_cache() {
    let db = init().await;
    let key = "a39e5954-a827-4467-8b7c-fbbe7fdfa567";
    let u = get_user(key, key, &db.conn).await.unwrap();
    info!("User: {u:?}");
  }
}
