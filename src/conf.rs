use std::fs;

use anyhow::anyhow;
use serde::{Deserialize, Serialize};

use crate::{cons, error::AppError};

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
  pub fn from_yaml(path: Option<&str>) -> Result<Self, AppError> {
    let p: String = path
      .map(|s| s.to_owned())
      .or_else(|| std::env::var_os(cons::ENV_CONFIG_PATH).and_then(|s| s.to_str().map(|s| s.to_owned())))
      .ok_or_else(|| {
        anyhow!(
          "Could not found configuration file path, pass [path] parameter or set environment [{}]",
          cons::ENV_CONFIG_PATH
        )
      })?;

    let content = fs::read_to_string(&p)?;
    let conf: Self = serde_yaml::from_str(&content).map_err(|e| anyhow!("Parse configuration file failed: {}", e))?;
    Ok(conf)
  }

  pub fn to_yaml(&self) -> String {
    serde_yaml::to_string(self).unwrap()
  }
}

impl Default for ServerConf {
  fn default() -> Self {
    Self {
      addr: cons::DEFAULT_SERVER_ADDR.into(),
    }
  }
}

#[cfg(test)]
mod tests {
  use std::fs::write;

  use super::*;

  #[test]
  fn test_to_yaml() {
    let conf = ApplicationConf::default();
    let ct: String = conf.to_yaml();
    write("./entry-config.yaml", ct).unwrap();
  }

  #[test]
  fn test_from_yaml() {
    let conf = ApplicationConf::from_yaml(Some("./entry-config.yaml"));
    conf.unwrap();
  }
}
