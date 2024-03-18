use axum::Router;

use crate::{error::AppError, middleware::response_wrapper::ApiResponse, state::AppState};
pub(crate) mod entity;
pub(crate) mod user;
pub(crate) use entity::prelude::*;

pub(crate) type Result<T> = std::result::Result<ApiResponse<T>, AppError>;

pub fn router() -> Router<AppState> {
  Router::new().nest("/v1/user", user::router())
}
