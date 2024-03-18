use anyhow::Ok;
use axum::Router;
use conf::ApplicationConf;
use db::DB;
use dotenvy::dotenv;
use log::info;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

mod conf;
mod cons;
mod db;
mod domain;
mod error;
mod logger;
mod middleware;
mod tool;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  dotenv().expect(".env file not found");
  logger::init();
  let conf = ApplicationConf::from_yaml(None)?;
  let db = DB::new(&conf).await?;

  let cors = CorsLayer::new().allow_methods(Any).allow_origin(Any);

  let app = Router::new()
    .fallback(middleware::resp_wrapper::handle_404)
    .nest("/api", domain::router())
    .layer(
      ServiceBuilder::new()
        .layer(cors)
        .layer(axum::middleware::from_fn(middleware::request_trace::layer)),
    );

  let listener = tokio::net::TcpListener::bind(&conf.server.addr).await?;
  info!("listening on {}", listener.local_addr().unwrap());
  axum::serve(listener, app).await?;
  Ok(())
}
