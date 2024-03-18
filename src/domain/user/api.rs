use crate::{domain::Result, middleware::response_wrapper::ApiResponse};

pub async fn index() -> Result<String> {
  let ret = ApiResponse::ok("User's domain".to_string());
  Ok(ret)
}
