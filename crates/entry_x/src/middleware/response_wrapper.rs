use axum::{http::StatusCode, response::IntoResponse, Json};
use log::{info, warn};

use crate::error::AppError;

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
  pub fn ok(data: T) -> Self {
    Self {
      code: 0,
      message: "ok".into(),
      data: Some(data),
    }
  }

  pub fn failed(err: AppError) -> Self {
    warn!("Response failed: {}", err);
    Self {
      code: err.code(),
      message: err.message(),
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
    let resp: ApiResponse<()> = ApiResponse::failed(self);
    info!("Response with error: {}", serde_json::to_string(&resp).unwrap());
    (StatusCode::OK, Json(resp)).into_response()
  }
}

pub async fn handle_404() -> impl IntoResponse {
  ApiResponse::<()>::failed(AppError::ApiNotFound)
}
