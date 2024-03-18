use axum::{extract::State, Json};
use sea_orm::ActiveValue::*;

use crate::{db::DB, domain::entity::users, error::AppError, middleware::response_wrapper::ApiResponse};

use super::{model::CreateReq, service::Mutation};

type Result<T> = std::result::Result<ApiResponse<T>, AppError>;

pub async fn index() -> Result<String> {
  let ret = ApiResponse::ok("User's domain".to_string());
  Ok(ret)
}

pub async fn create(State(db): State<DB>, Json(payload): Json<CreateReq>) -> Result<users::Model> {
  // TODO: Valid request
  let act_model = users::ActiveModel {
    email: Set(payload.email),
    password: Set(payload.password),
    hash: Set(payload.hash),
    name: match payload.name {
      Some(v) => Set(Some(v)),
      None => NotSet,
    },
    ..Default::default()
  };

  let ret = Mutation::create(&db.conn, act_model).await?;
  Ok(ApiResponse::ok(ret))
}
