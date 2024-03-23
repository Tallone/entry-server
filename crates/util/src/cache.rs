use std::{env, time::Duration};

use async_trait::async_trait;
use fred::{
  clients::RedisPool,
  interfaces::{ClientLike, KeysInterface},
  types::{Expiration, ReconnectPolicy, RedisConfig},
};
use serde::Serialize;
use tokio::sync::OnceCell;

const ENV_REDIS_URL: &str = "ENTRY_CACHE_REDIS_URL";
const POOL_SIZE: usize = 4;
static INSTANCE: OnceCell<RedisPool> = OnceCell::const_new();

#[async_trait]
pub trait Cache {
  async fn set<V: Serialize + Send + Sync>(&self, key: &str, value: V, expire: Option<Duration>);
  async fn get(key: &str);
  async fn del(key: &str);
}

/// Get an `RedisClient`
pub async fn redis() -> RedisPool {
  INSTANCE
    .get_or_init(|| async {
      let url = env::var(ENV_REDIS_URL).expect(format!("Missing environment: {}", ENV_REDIS_URL).as_str());
      let config = RedisConfig::from_url(&url).unwrap();
      let reconnect = ReconnectPolicy::new_constant(5, 3000);
      let pool = RedisPool::new(config, None, None, Some(reconnect), POOL_SIZE).unwrap();
      pool.connect();
      pool.wait_for_connect().await.unwrap();
      pool
    })
    .await
    .clone()
}

pub struct RedisCache{
  pool: RedisPool
}

#[async_trait]
impl Cache for RedisCache {
    async fn set<V: Serialize + Send + Sync>(&self, key: &str, value: V, expire: Option<Duration>) {
    let _ = self.pool.set(key, value, expire.map(|v| Expiration::PXAT(v.as_millis() as i64)), None, true);
    }

    async fn get(key: &str) {
        todo!()
    \}

    async fn del(key: &str) {
        todo!()
    \}


}
