use anyhow::Ok;
use axum::{error_handling::HandleErrorLayer, Router};
use conf::ApplicationConf;
use db::DB;
use dotenvy::dotenv;
use env_logger::Env;
use log::info;

mod conf;
mod cons;
mod db;
mod domain;
mod error;
mod middleware;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  dotenv().expect(".env file not found");
  env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
  let conf = ApplicationConf::from_yaml(None)?;
  let db = DB::new(&conf).await?;

  let app = Router::new().nest("/api", domain::router());
  let app = app.fallback(middleware::resp_wrapper::handle_404);

  let listener = tokio::net::TcpListener::bind(&conf.server.addr).await?;
  info!("listening on {}", listener.local_addr().unwrap());
  axum::serve(listener, app).await?;
  Ok(())
}
