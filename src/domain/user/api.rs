use axum::{extract::State, Json};

use crate::{
  db::DB,
  domain::{Result, Users},
  middleware::response_wrapper::ApiResponse,
};

use super::model::CreateReq;

pub async fn index() -> Result<String> {
  let ret = ApiResponse::ok("User's domain".to_string());
  Ok(ret)
}
