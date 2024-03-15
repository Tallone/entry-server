use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::{conf::ApplicationConf, error::Result};

pub struct DB {
  pub pool: Pool<Postgres>,
}

impl DB {
  pub async fn new(conf: &ApplicationConf) -> Result<Self> {
    let pool = PgPoolOptions::new().max_connections(20).connect(&conf.db.url).await?;
    Ok(Self { pool: pool })
  }
}
