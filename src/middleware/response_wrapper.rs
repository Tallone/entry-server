use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::error::AppError;

// API response structure
#[derive(serde::Serialize)]
pub struct ApiResponse<T> {
  code: u32,
  message: String,
  data: T,
}

impl<T> ApiResponse<T>
where
  T: serde::Serialize + Default,
{
  pub fn ok(data: T) -> Self {
    Self {
      code: 0,
      message: "ok".into(),
      data,
    }
  }

  pub fn failed(err: AppError) -> Self {
    Self {
      code: err.code(),
      message: err.message(),
      data: T::default(),
    }
  }
}

// Implement `IntoResponse` for `ApiError`
impl<T> IntoResponse for ApiResponse<T>
where
  T: serde::Serialize + Default,
{
  fn into_response(self) -> axum::response::Response {
    (StatusCode::OK, Json(self)).into_response()
  }
}

impl IntoResponse for AppError {
  fn into_response(self) -> axum::response::Response {
    let resp: ApiResponse<()> = ApiResponse::failed(self);
    (StatusCode::OK, Json(resp)).into_response()
  }
}

pub async fn handle_404() -> impl IntoResponse {
  ApiResponse::<()>::failed(AppError::ApiNotFound)
}
