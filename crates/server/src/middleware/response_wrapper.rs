use axum::{
  extract::rejection::{JsonRejection, PathRejection, QueryRejection},
  http::StatusCode,
  response::IntoResponse,
};
use axum_macros::{FromRequest, FromRequestParts};
use log::info;
use serde::Serialize;

use crate::internal::error::AppError;

// API response structure
#[derive(serde::Serialize)]
pub struct ApiResponse<T> {
  code: u32,
  message: String,
  data: Option<T>,
}

impl<T> ApiResponse<T>
where
  T: serde::Serialize,
{
  pub fn ok(data: T) -> Result<Self, AppError> {
    Ok(Self {
      code: 0,
      message: "ok".into(),
      data: Some(data),
    })
  }
}

impl From<AppError> for ApiResponse<()> {
  fn from(value: AppError) -> Self {
    Self {
      code: value.code(),
      message: value.message(),
      data: None,
    }
  }
}

// Implement `IntoResponse` for `ApiError`
impl<T> IntoResponse for ApiResponse<T>
where
  T: serde::Serialize,
{
  fn into_response(self) -> axum::response::Response {
    (StatusCode::OK, Json(self)).into_response()
  }
}

impl IntoResponse for AppError {
  fn into_response(self) -> axum::response::Response {
    info!("Response error: {}", self.to_string());
    let resp: ApiResponse<()> = self.into();
    (StatusCode::OK, Json(resp)).into_response()
  }
}

pub async fn handle_404() -> impl IntoResponse {
  AppError::ApiNotFound
}

#[derive(FromRequestParts)]
#[from_request(via(axum::extract::Path), rejection(AppError))]
pub struct Path<T>(pub T);
impl From<PathRejection> for AppError {
  fn from(_: PathRejection) -> Self {
    AppError::ApiNotFound
  }
}

#[derive(FromRequestParts)]
#[from_request(via(axum::extract::Query), rejection(AppError))]
pub struct Query<T>(pub T);
impl From<QueryRejection> for AppError {
  fn from(value: QueryRejection) -> Self {
    AppError::RequestNotValid(value.to_string())
  }
}

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(AppError))]
pub struct Json<T>(pub T);
impl<T: Serialize> IntoResponse for Json<T> {
  fn into_response(self) -> axum::response::Response {
    let Self(value) = self;
    axum::Json(value).into_response()
  }
}
impl From<JsonRejection> for AppError {
  fn from(value: JsonRejection) -> Self {
    AppError::RequestNotValid(value.to_string())
  }
}
