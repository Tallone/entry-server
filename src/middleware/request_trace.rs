use axum::{
  body::Body,
  extract::Request,
  http::{header::CONTENT_TYPE, HeaderValue},
  middleware::Next,
  response::IntoResponse,
};
use http_body_util::BodyExt;
use log::{info, warn};
use serde_json::Value;
use time::Instant;
use uuid::Uuid;

use crate::{cons, error::AppError, middleware::response_wrapper::ApiResponse};

/// This middleware generate a [`HEADER_REQUEST_ID`] to headers,
/// And will logging request body if the request content type is json.
pub async fn layer(req: Request, next: Next) -> Result<impl IntoResponse, ApiResponse<()>> {
  let start = Instant::now();
  let req_id = Uuid::new_v4().to_string();
  let (mut parts, body) = req.into_parts();
  parts
    .headers
    .insert(cons::HEADER_REQUEST_ID, HeaderValue::from_str(&req_id).unwrap());

  let is_multipart_request = parts
    .headers
    .get(CONTENT_TYPE)
    .map_or(false, |v| v.to_str().unwrap_or_default().starts_with("multipart"));

  let body_string: String = match is_multipart_request {
    true => "".into(),
    false => {
      let bytes = body
        .collect()
        .await
        .map_err(|e| {
          warn!("Collect request body failed {}", e);
          ApiResponse::failed(AppError::RequestNotValid)
        })?
        .to_bytes();
      let val: Value = serde_json::from_slice(&bytes).map_err(|_| ApiResponse::failed(AppError::RequestNotValid))?;
      val.to_string()
    }
  };

  // [request id] method path params
  info!(
    "[{}] {} {} {}",
    req_id,
    parts.method.to_string(),
    parts.uri.to_string(),
    body_string
  );

  let request = Request::from_parts(parts, Body::from(body_string));
  let resp = next.run(request).await;

  info!("[{}] done {}ms", req_id, start.elapsed().whole_milliseconds());
  Ok(resp)
}
