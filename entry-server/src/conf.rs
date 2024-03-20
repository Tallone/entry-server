use serde::{Deserialize, Serialize};

use crate::cons;

#[derive(Default, Serialize, Deserialize)]
pub struct ApplicationConf {
  pub server: ServerConf,
  pub db: DatabaseConf,
}

#[derive(Default, Serialize, Deserialize)]
pub struct DatabaseConf {
  pub url: String,
}

/// Server configuration
#[derive(Serialize, Deserialize)]
pub struct ServerConf {
  pub addr: String,
}

impl ApplicationConf {
  pub fn from_env() -> Self {
    Self {
      db: DatabaseConf {
        url: std::env::var(cons::ENV_DATABASE_URL).expect(format!("Envirement {} is not valid", cons::ENV_DATABASE_URL).as_str()),
      },
      server: ServerConf {
        addr: std::env::var(cons::ENV_SERVER_ADDR).expect(format!("Envirement {} is not valid", cons::ENV_SERVER_ADDR).as_str()),
      }
    }
  }
}

impl Default for ServerConf {
  fn default() -> Self {
    Self {
      addr: cons::DEFAULT_SERVER_ADDR.into(),
    }
  }
}

