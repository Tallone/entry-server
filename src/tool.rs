use anyhow::anyhow;
use argon2::password_hash::{PasswordHasher, SaltString};
use argon2::Argon2;
use once_cell::sync::Lazy;
use time::{macros::format_description, OffsetDateTime, UtcOffset};
use tokio::sync::OnceCell;

use crate::cons;

/// Get current timestamp in milliseconds
pub fn current_ms() -> u64 {
  let dt = OffsetDateTime::now_utc().to_offset(UtcOffset::from_hms(cons::DEFAULT_TIME_OFFSET, 0, 0).unwrap());
  dt.unix_timestamp_nanos() as u64 / 1_000_000
}

/// Get current formatted datetime
pub fn current_time() -> String {
  let dt = OffsetDateTime::now_utc().to_offset(UtcOffset::from_hms(cons::DEFAULT_TIME_OFFSET, 0, 0).unwrap());
  let fmt = format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");
  dt.format(fmt).unwrap()
}

pub fn hash_argon2(input: &str, salt: &str) -> anyhow::Result<String> {
  let hasher = Argon2::default();
  let salt = SaltString::from_b64(salt).map_err(|e| anyhow!("Hash salt is not valid {}", e))?;
  let ret = hasher
    .hash_password(input.as_bytes(), &salt)
    .map_err(|e| anyhow!("Hash failed {}", e))?;
  Ok(ret.to_string())
}

#[cfg(test)]
mod test {
  use super::*;
  use log::info;
  use rand::rngs::OsRng;

  fn init() {
    let _ = env_logger::builder()
      .filter_level(log::LevelFilter::Info)
      .is_test(true)
      .try_init();
  }

  #[test]
  fn test_hash_argon2() {
    init();
    let salt = SaltString::generate(&mut OsRng);
    let salt = "q7B6QeFR0inm+oB1oBuAYQ";
    let ret = "$argon2id$v=19$m=19456,t=2,p=1$q7B6QeFR0inm+oB1oBuAYQ$WioZWSYdwUNqvLeYm1dDiq+vEt4Xfbd1XH3yE28un3E";
    assert_eq!(hash_argon2("122333", salt).unwrap(), ret);
  }
}
