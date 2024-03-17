use sea_orm::{Database, DatabaseConnection};

use crate::{conf::ApplicationConf, error::AppError};

pub struct DB {
  pub conn: DatabaseConnection,
}

impl DB {
  pub async fn new(conf: &ApplicationConf) -> Result<Self, AppError> {
    let conn = Database::connect(&conf.db.url).await?;

    Ok(Self { conn })
  }
}
