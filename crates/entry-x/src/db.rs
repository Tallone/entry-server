use std::time::Duration;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use crate::{conf::ApplicationConf, error::AppError};

#[derive(Clone)]
pub struct DB {
  pub conn: DatabaseConnection,
}

impl DB {
  pub async fn new(conf: &ApplicationConf) -> Result<Self, AppError> {
    let mut opt = ConnectOptions::new(&conf.db.url);
    opt.connect_timeout(Duration::from_secs(1));
    opt.sqlx_logging(false);
    let conn = Database::connect(opt).await?;

    Ok(Self { conn })
  }
}
