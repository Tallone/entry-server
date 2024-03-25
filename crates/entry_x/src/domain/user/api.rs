use anyhow::anyhow;
use axum::{
  extract::{Path, State},
  Json,
};
use sea_orm::ActiveValue::*;
use uuid::Uuid;

use crate::{db::DB, domain::entity::users, error::AppError, middleware::response_wrapper::ApiResponse};

use super::{
  model::{CreateReq, GetReq},
  service::{Mutation, Query},
};

type Result<T> = std::result::Result<ApiResponse<T>, AppError>;

pub async fn create(State(db): State<DB>, Json(payload): Json<CreateReq>) -> Result<users::Model> {
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

pub async fn get(State(db): State<DB>, Path(id): Path<String>) -> Result<users::Model> {
  let id = Uuid::try_parse(&id).map_err(|_| anyhow!("Id is not valid uuid"))?;
  let ret = Query::get(&db.conn, GetReq::Id(id)).await?;
  Ok(ApiResponse::ok(ret))
}
