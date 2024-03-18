use axum::Router;

use crate::{error::AppError, middleware::resp_wrapper::ApiResponse};
pub(crate) mod entity;
pub(crate) mod user;

pub(crate) type Result<T> = std::result::Result<ApiResponse<T>, AppError>;

pub fn router() -> Router {
  Router::new().nest("/v1/user", user::router())
}
