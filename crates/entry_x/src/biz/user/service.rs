use std::time::Duration;

use anyhow::anyhow;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use util::{Expiration, KeysInterface};
use uuid::Uuid;

use crate::{
  biz::{entity::users, Result},
  gen_crud,
};

use super::model::GetReq;

const DEFAULT_CACHE_DURATION: Duration = Duration::from_secs(2 * 30 * 60);

gen_crud!(users, users::Column::Id);

impl Query {
  pub async fn get_opt(conn: &DatabaseConnection, opt: GetReq) -> Result<Option<users::Model>> {
    let query = users::Entity::find();
    let query = match opt {
      GetReq::Id(id) => query.filter(users::Column::Id.eq(id)),
      GetReq::Email(v) => query.filter(users::Column::Email.eq(v)),
    };
    let resp = query.one(conn).await?;
    Ok(resp)
  }
}

const USER_CACHE_PREFIX: &str = "USER_";
pub async fn get_user(id: &str, conn: &DatabaseConnection) -> Result<Option<users::Model>> {
  let key = format!("{}{}", USER_CACHE_PREFIX, id);
  let redis = util::cache::redis().await;
  let count: usize = redis.exists(&key).await?;
  if count > 0 {
    let json: String = redis.get(&key).await?;
    let u: users::Model = serde_json::from_str(&json).map_err(|_| anyhow!("Got an invlid user json from redis"))?;
    return Ok(Some(u));
  }

  let uid = Uuid::parse_str(id).map_err(|_| anyhow!("Not a valid uuid: {}", id))?;
  if let Some(u) = users::Entity::find_by_id(uid).one(conn).await? {
    let data = serde_json::to_string(&u).unwrap();
    redis
      .set(
        &key,
        &data,
        Some(Expiration::EX(DEFAULT_CACHE_DURATION.as_secs() as i64)),
        None,
        false,
      )
      .await?;

    return Ok(Some(u));
  }
  Ok(None)
}

const TOKEN_CACHE_PREFIX: &str = "TOKEN_";

/// Generate a token for user `id` and store it in cache
pub async fn create_token(id: &str) -> Result<String> {
  let token = util::rand_str(64);
  let key = format!("{}{}", TOKEN_CACHE_PREFIX, token);
  let redis = util::cache::redis().await;
  redis
    .set(
      &key,
      id,
      Some(Expiration::EX(DEFAULT_CACHE_DURATION.as_secs() as i64)),
      None,
      false,
    )
    .await?;
  Ok(token)
}

pub async fn get_user_by_token(token: &str, conn: &DatabaseConnection) -> Result<Option<users::Model>> {
  let key = format!("{}{}", TOKEN_CACHE_PREFIX, token);
  let redis = util::cache::redis().await;
  let uid: Option<String> = redis.get(&key).await?;
  match uid {
    Some(v) => return get_user(&v, &conn).await,
    None => return Ok(None),
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use dotenvy::dotenv;
  use log::info;

  use crate::internal::{conf::ApplicationConf, db::DB};

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
    let u = get_user(key, &db.conn).await.unwrap();
    info!("User: {u:?}");
  }
}
