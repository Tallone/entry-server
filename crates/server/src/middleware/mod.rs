use axum::http::{HeaderName, HeaderValue};
use axum::response::IntoResponse;
use axum::Router;
use log::error;
use tower::ServiceBuilder;
use tower_http::catch_panic::CatchPanicLayer;
use tower_http::cors::{Any, CorsLayer};
use tower_http::propagate_header::PropagateHeaderLayer;
use tower_http::set_header::SetRequestHeaderLayer;
use uuid::Uuid;

use crate::cons;
use crate::internal::error::AppError;

pub mod authenticator;
pub mod request_trace;
pub mod response_wrapper;

pub fn init(router: Router) -> Router {
  let layers = ServiceBuilder::new()
    .layer(CorsLayer::new().allow_methods(Any).allow_origin(Any))
    .layer(SetRequestHeaderLayer::if_not_present(
      cons::HEADER_REQUEST_ID.try_into().unwrap(),
      HeaderValue::from_str(&Uuid::new_v4().to_string()).unwrap(),
    ))
    .layer(PropagateHeaderLayer::new(HeaderName::from_static(
      cons::HEADER_REQUEST_ID,
    )))
    .layer(CatchPanicLayer::custom(handle_panic))
    .layer(axum::middleware::from_fn(request_trace::layer));

  router.layer(layers)
}

fn handle_panic(err: Box<dyn std::any::Any + Send + 'static>) -> axum::response::Response {
  let details = if let Some(s) = err.downcast_ref::<String>() {
    s.clone()
  } else if let Some(s) = err.downcast_ref::<&str>() {
    s.to_string()
  } else {
    "Unknown panic message".to_string()
  };
  error!("panic: {}", details);
  AppError::Unknown("panic".to_string()).into_response()
}
