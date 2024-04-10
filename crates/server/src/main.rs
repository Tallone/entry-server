use std::{net::SocketAddr, time::Duration};

use anyhow::Ok;
use axum::Router;
use dotenvy::dotenv;
use log::info;
use tokio::sync::broadcast;

use crate::internal::{app_state::AppState, conf::ApplicationConf, db::DB, logger, router_tree::RouteNode};

mod biz;
mod cons;
mod internal;
mod middleware;

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

  let state = AppState { db };

  let mut api_tree = RouteNode::new("/api");

  let app = Router::new()
    .nest("/api", biz::router())
    .with_state(state)
    .fallback(middleware::response_wrapper::handle_404);
  let app = middleware::init(app);

  let listener = tokio::net::TcpListener::bind(&conf.server.addr).await?;
  info!("listening on {}", listener.local_addr().unwrap());
  axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await?;
  Ok(())
}
