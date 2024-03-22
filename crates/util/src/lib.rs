pub mod cache;
pub mod http;

use uuid::Uuid;

/// Obtain a random UUID string
pub fn rand_uuid() -> String {
  Uuid::new_v4().to_string()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_rand_uuid() {
    let v = rand_uuid();
    assert!(!v.is_empty());
  }
}
