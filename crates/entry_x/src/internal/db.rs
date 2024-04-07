use sea_orm::{ColumnTrait, ConnectOptions, Database, DatabaseConnection, Order};
use std::time::Duration;

use super::{conf::ApplicationConf, error::AppError};

pub struct ColumnOrder<T: ColumnTrait> {
  pub column: T,
  pub order: Order,
}

#[derive(Clone)]
pub struct DB {
  pub conn: DatabaseConnection,
}

impl DB {
  pub async fn new(conf: &ApplicationConf) -> Result<Self, AppError> {
    let mut opt = ConnectOptions::new(&conf.db.url);
    opt.connect_timeout(Duration::from_secs(3));
    opt.sqlx_logging(false);
    let conn = Database::connect(opt).await?;

    Ok(Self { conn })
  }
}
