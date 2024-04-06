use std::time::Duration;

use anyhow::Ok;
use axum::{Router};
use conf::ApplicationConf;
use db::DB;
use dotenvy::dotenv;
use log::info;
use tokio::sync::broadcast;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

mod conf;
mod cons;
mod db;
mod domain;
mod error;
mod logger;
mod middleware;
mod state;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  dotenv().expect(".env file not found");

  // Initialize logger
  logger::init();

  // Parse configuration from environment
  let conf = ApplicationConf::from_env();

  // Connect to database
  let db = DB::new(&conf).await?;

  // The shutdown channel
  let (_, rx) = broadcast::channel::<()>(8);

  // Start task system
  task::start_tick(Duration::from_millis(100), rx);

  let state = state::AppState { db };
  let cors = CorsLayer::new().allow_methods(Any).allow_origin(Any);
  let app = Router::new()
    .nest("/api", domain::router())
    .with_state(state)
    .fallback(middleware::response_wrapper::handle_404)
    .layer(
      ServiceBuilder::new()
        .layer(cors)
        .layer(axum::middleware::from_fn(middleware::request_trace::layer))
    );

  let listener = tokio::net::TcpListener::bind(&conf.server.addr).await?;
  info!("listening on {}", listener.local_addr().unwrap());
  axum::serve(listener, app).await?;
  Ok(())
}
