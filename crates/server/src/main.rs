use std::{net::SocketAddr, time::Duration};

use anyhow::Ok;
use axum::{
  http::{HeaderName, HeaderValue},
  Router,
};
use dotenvy::dotenv;
use log::info;
use tokio::sync::broadcast;
use tower::ServiceBuilder;
use tower_http::{
  cors::{Any, CorsLayer},
  propagate_header::PropagateHeaderLayer,
  set_header::SetRequestHeaderLayer,
};
use uuid::Uuid;

use crate::internal::{app_state::AppState, conf::ApplicationConf, db::DB, logger};

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
  let cors = CorsLayer::new().allow_methods(Any).allow_origin(Any);
  let app = Router::new()
    .nest("/api", biz::router())
    .with_state(state)
    .fallback(middleware::response_wrapper::handle_404)
    .layer(
      ServiceBuilder::new()
        .layer(cors)
        .layer(SetRequestHeaderLayer::if_not_present(
          cons::HEADER_REQUEST_ID.try_into().unwrap(),
          HeaderValue::from_str(&Uuid::new_v4().to_string()).unwrap(),
        ))
        .layer(PropagateHeaderLayer::new(HeaderName::from_static(
          cons::HEADER_REQUEST_ID,
        )))
        .layer(axum::middleware::from_fn(middleware::request_trace::layer)),
    );

  let listener = tokio::net::TcpListener::bind(&conf.server.addr).await?;
  info!("listening on {}", listener.local_addr().unwrap());
  axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await?;
  Ok(())
}
