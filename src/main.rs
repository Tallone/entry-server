use conf::ApplicationConf;
use db::DB;
use dotenvy::dotenv;

mod conf;
mod cons;
mod db;
mod error;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  dotenv().expect(".env file not found");
  let conf = ApplicationConf::from_yaml(None)?;
  let db = DB::new(&conf).await?;

  sqlx::migrate!().run(&db.pool).await?;
  Ok(())
}
