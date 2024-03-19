use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::{domain::entity::users, error::AppError};
use users::Entity as Users;

use super::model::GetReq;

pub(crate) struct Mutation;
pub(crate) struct Query;

type Result<T> = std::result::Result<T, AppError>;

impl Mutation {
  pub async fn create(db: &DatabaseConnection, model: users::ActiveModel) -> Result<users::Model> {
    let ret = model.insert(db).await?;
    Ok(ret)
  }
}

impl Query {
  pub async fn get(db: &DatabaseConnection, opt: GetReq) -> Result<users::Model> {
    let query = Users::find();
    let query = match opt {
      GetReq::Id(id) => query.filter(users::Column::Id.eq(id)),
      GetReq::Email(v) => query.filter(users::Column::Email.eq(v)),
    };
    let resp = query.one(db).await?;
    resp.ok_or(AppError::RecordNotFound)
  }
}
