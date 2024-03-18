use std::io;

use thiserror::Error;

use crate::cons;

#[derive(Error, Debug)]
pub enum AppError {
  #[error(transparent)]
  Biz(#[from] anyhow::Error),

  #[error("Api is not found")]
  ApiNotFound,

  #[error("Request content is not valid")]
  RequestNotValid,

  #[error("IO Error: {0}")]
  IO(#[from] io::Error),

  #[error("DB Error: {0}")]
  Db(#[from] sea_orm::DbErr),

  #[error("Unhandled internal error")]
  Unknown,
}

impl AppError {
  pub fn code(&self) -> u32 {
    match self {
      AppError::Biz(_) => 4000,
      AppError::RequestNotValid => 4010,
      AppError::IO(_) => 1500,
      AppError::Db(_) => 1510,
      AppError::Unknown => 9999,
      AppError::ApiNotFound => 404,
    }
  }

  pub fn message(&self) -> String {
    match self {
      AppError::IO(_) | AppError::Db(_) => cons::MSG_INTERNAL_ERROR.to_string(),
      _ => self.to_string(),
    }
  }
}
