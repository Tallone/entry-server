use axum::Router;

use crate::internal::{app_state::AppState, error::AppError};
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
