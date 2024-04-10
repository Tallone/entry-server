use axum::{handler::Handler, http::Method, Router};

use crate::internal::{app_state::AppState, error::AppError, router_tree::RouteNode};
pub(crate) mod activation;
pub(crate) mod entity;
pub(crate) mod license;
pub(crate) mod macros;
pub(crate) mod synchronize;
pub(crate) mod user;

pub(crate) type Result<T> = std::result::Result<T, AppError>;

pub fn router() -> Router<AppState> {
  Router::new()
    .nest("/v1/user", user::router())
    .nest("/v1/license", license::router())
    .nest("/v1/sync", synchronize::router())
}

pub fn init<H, T>(root: &mut RouteNode<H, AppState>)
where
  H: Handler<T, AppState>,
  T: 'static,
{
  root.path("/v1/license").nest(license::router());
}

