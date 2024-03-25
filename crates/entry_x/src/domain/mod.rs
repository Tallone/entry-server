use axum::Router;

use crate::{error::AppError, state::AppState};
pub mod activation;
pub(crate) mod entity;
pub mod license;
pub(crate) mod user;
pub(crate) type Result<T> = std::result::Result<T, AppError>;

pub fn router() -> Router<AppState> {
  Router::new().nest("/v1/user", user::router())
}
