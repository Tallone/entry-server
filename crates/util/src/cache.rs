use std::{env, time::Duration};

use fred::{
  clients::RedisPool,
  interfaces::ClientLike,
  types::{ReconnectPolicy, RedisConfig},
};
use tokio::sync::OnceCell;

const ENV_REDIS_URL: &str = "REDIS_URL";
const POOL_SIZE: usize = 4;
const DEFAULT_EXPIRE_DURATION: Duration = Duration::from_secs(2 * 60 * 60);
static INSTANCE: OnceCell<RedisPool> = OnceCell::const_new();

/// Get an `RedisClient`
pub async fn redis() -> RedisPool {
  INSTANCE
    .get_or_init(|| async {
      let url = env::var(ENV_REDIS_URL).expect(format!("Missing environment {}", ENV_REDIS_URL).as_str());
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
