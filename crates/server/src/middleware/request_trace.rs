use axum::{
  body::Body,
  extract::Request,
  http::{header::CONTENT_TYPE, Method},
  middleware::Next,
  response::IntoResponse,
};
use http_body_util::BodyExt;
use log::{info, warn};
use serde_json::Value;
use time::Instant;

use crate::{cons, internal::error::AppError};

use super::response_wrapper::ApiResponse;

/// This middleware generate a [`HEADER_REQUEST_ID`] to headers,
/// And will logging request body if the request content type is json.
pub async fn layer(req: Request, next: Next) -> Result<impl IntoResponse, ApiResponse<()>> {
  let start = Instant::now();
  let (parts, body) = req.into_parts();
  let req_id: String = parts
    .headers
    .get(cons::HEADER_REQUEST_ID)
    .ok_or(AppError::RequestNotValid)?
    .to_str()
    .map_err(|_| AppError::RequestNotValid)?
    .to_owned();

  let is_multipart_request = parts
    .headers
    .get(CONTENT_TYPE)
    .map_or(false, |v| v.to_str().unwrap_or_default().starts_with("multipart"));
  let should_print_body = !is_multipart_request && parts.method != Method::GET;

  let body_string: String = match should_print_body {
    false => "".into(),
    true => {
      let bytes = body
        .collect()
        .await
        .map_err(|e| {
          warn!("Collect request body failed {}", e);
          AppError::RequestNotValid
        })?
        .to_bytes();
      let val: Value = serde_json::from_slice(&bytes).map_err(|_| AppError::RequestNotValid)?;
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
