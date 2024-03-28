use base64ct::{Base64, Encoding};
pub use fred::prelude::*;
pub mod cache;
pub mod http;

use sha2::{Digest, Sha256};
use uuid::Uuid;

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
