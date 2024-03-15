use std::io;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, ApplicationError>;

#[derive(Error, Debug)]
pub enum ApplicationError {
  #[error(transparent)]
  Other(#[from] anyhow::Error),

  // #[error("The file path '{0}' could not be found")]
  // FileNotFound(String),
  #[error("IO Error: {0}")]
  IO(#[from] io::Error),
}
