use sea_orm::{ActiveModelTrait, DatabaseConnection};
use uuid::Uuid;

use crate::{domain::entity::users, error::AppError};

pub(crate) struct Mutation;

type Result<T> = std::result::Result<T, AppError>;

impl Mutation {
  pub async fn create(db: &DatabaseConnection, model: users::ActiveModel) -> Result<users::Model> {
    let ret = model.insert(db).await?;
    Ok(ret)
  }
}
