use error::AppError;

pub mod error;

pub type Result<T> = std::result::Result<T, AppError>;