use anyhow::anyhow;
use argon2::{password_hash::SaltString, Argon2};
use argon2::{PasswordHash, PasswordHasher, PasswordVerifier};
use base64ct::{Base64, Encoding};
pub use fred::prelude::*;
pub mod cache;
pub mod http;

use rand::rngs::OsRng;
use sha2::{Digest, Sha256};
use time::macros::format_description;
use time::{OffsetDateTime, UtcOffset};
use uuid::Uuid;

pub const DEFAULT_TIME_OFFSET: i8 = 8;

/// Obtain a random UUID string
pub fn rand_uuid() -> String {
  Uuid::new_v4().to_string()
}

/// Generate a random `len` string
pub fn rand_str(len: usize) -> String {
  let seed = rand_uuid();
  let hash = Sha256::new().chain_update(seed).finalize();
  Base64::encode_string(&hash).chars().take(len).collect()
}

/// Get current timestamp in milliseconds
pub fn current_ms() -> u64 {
  let dt = OffsetDateTime::now_utc().to_offset(UtcOffset::from_hms(DEFAULT_TIME_OFFSET, 0, 0).unwrap());
  dt.unix_timestamp_nanos() as u64 / 1_000_000
}

/// Get current formatted datetime
pub fn current_time() -> String {
  let dt = OffsetDateTime::now_utc().to_offset(UtcOffset::from_hms(DEFAULT_TIME_OFFSET, 0, 0).unwrap());
  let fmt = format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");
  dt.format(fmt).unwrap()
}

/// Encrypt password using argon algorithmic
pub fn argon2_encrypt(passwd: &str) -> anyhow::Result<String> {
  let h = Argon2::default();
  let salt = SaltString::generate(&mut OsRng);
  let hashed = h
    .hash_password(passwd.as_bytes(), &salt)
    .map_err(|e| anyhow!("argon2 encrpyt error: {}", e))?
    .to_string();
  Ok(hashed)
}

pub fn argon2_verify(plain_text: &str, hashed: &str) -> anyhow::Result<()> {
  let ph = PasswordHash::new(hashed).map_err(|_| anyhow!("Not a valid password"))?;
  Argon2::default()
    .verify_password(plain_text.as_bytes(), &ph)
    .map_err(|_| anyhow!("Verify failed"))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_rand_uuid() {
    let v = rand_uuid();
    assert!(!v.is_empty());
  }

  #[test]
  fn test_rand_str() {
    assert_eq!(rand_str(10).len(), 10);
  }
}
