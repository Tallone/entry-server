use axum::{extract::State, Json};
use sea_orm::ActiveValue::*;

use crate::{
  db::DB,
  domain::entity::users,
  error::AppError,
  middleware::{authenticator::LoginedUser, response_wrapper::ApiResponse},
};

use super::{model::CreateReq, service::Mutation};

type Result<T> = std::result::Result<ApiResponse<T>, AppError>;

pub async fn create(State(db): State<DB>, Json(payload): Json<CreateReq>) -> Result<users::Model> {
  let act_model = users::ActiveModel {
    email: Set(payload.email),
    password: Set(Some(payload.password)),
    hash: Set(Some(payload.hash)),
    name: match payload.name {
      Some(v) => Set(Some(v)),
      None => NotSet,
    },
    ..Default::default()
  };

  let ret = Mutation::create(&db.conn, act_model).await?;
  Ok(ApiResponse::ok(ret))
}

/// Get current login user info
pub async fn current(user: LoginedUser) -> Result<users::Model> {
  Ok(ApiResponse::ok(user.0))
}
