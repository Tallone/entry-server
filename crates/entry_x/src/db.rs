use sea_orm::{ColumnTrait, ConnectOptions, Database, DatabaseConnection, Order};
use std::time::Duration;

use crate::{conf::ApplicationConf, error::AppError};

pub struct ColumnOrder<T: ColumnTrait> {
  pub column: T,
  pub order: Order,
}

#[derive(Clone)]
pub struct DB {
  pub conn: DatabaseConnection,
}

impl DB {
  pub async fn new(conf: &ApplicationConf) -> Result<Self, AppError> {
    let mut opt = ConnectOptions::new(&conf.db.url);
    opt.connect_timeout(Duration::from_secs(1));
    opt.sqlx_logging(false);
    let conn = Database::connect(opt).await?;

    Ok(Self { conn })
  }
}

#[macro_export]
macro_rules! gen_crud {
  ($entity:ident) => {
    use crate::db::ColumnOrder;
    use sea_orm::prelude::*;
    use sea_orm::{QueryOrder, QueryTrait};
    pub(crate) struct Mutation;
    pub(crate) struct Query;

    impl Mutation {
      pub async fn create(
        db: sea_orm::DatabaseConnection,
        model: $entity::ActiveModel,
      ) -> Result<$entity::Model, sea_orm::DbErr> {
        let resp = model.insert(&db).await?;
        Ok(resp)
      }

      pub async fn update(
        db: sea_orm::DatabaseConnection,
        model: $entity::ActiveModel,
      ) -> Result<$entity::Model, sea_orm::DbErr>
      {
        let ret = $entity::Entity::update(model)
          .exec(&db)
          .await?;
        Ok(ret)
      }

      // Deletes a record based on the `model`
      //
      // Returns true if the deletion was successful, false if no records were deleted.
      pub async fn delete_one(
        db: sea_orm::DatabaseConnection,
        model: $entity::ActiveModel,
      ) -> Result<bool, sea_orm::DbErr> {
        let ret = $entity::Entity::delete(model).exec(&db).await?;
        Ok(ret.rows_affected > 0)
      }
    }

    impl Query {
      // Convenient way to get `Select`
      fn get_select() -> sea_orm::Select<$entity::Entity> {
        $entity::Entity::find()
      }

      // Retrieves a record from the database based on a specified column and value.
      pub async fn get<V>(
        db: sea_orm::DatabaseConnection,
        column: $entity::Column,
        v: V,
      ) -> Result<Option<$entity::Model>, sea_orm::DbErr>
      where
        V: Into<sea_orm::Value>,
      {
        let resp = Self::get_select().filter(column.eq(v)).one(&db).await?;
        Ok(resp)
      }

      // Retrieves a record from the database by id.
      pub async fn get_by_id<T>(
        db: sea_orm::DatabaseConnection,
        id: T,
      ) -> Result<Option<$entity::Model>, sea_orm::DbErr>
      where
        T: Into<<$entity::PrimaryKey as sea_orm::PrimaryKeyTrait>::ValueType>,
      {
        Self::get(db, $entity::Column::Id, id.into()).await
      }

      // Retrieves a list of records from the database
      // based on `column` and `values`.
      pub async fn list_in<V>(
        db: sea_orm::DatabaseConnection,
        column: $entity::Column,
        values: Vec<V>,
        order: Option<ColumnOrder<$entity::Column>>,
      ) -> Result<Vec<$entity::Model>, sea_orm::DbErr>
      where
        V: Into<sea_orm::Value>,
      {
        let mut filters = sea_orm::Condition::all();
        filters = filters.add(column.is_in(values));
        let resp = $entity::Entity::find()
          .filter(filters)
          .apply_if(order, |q, v| q.order_by(v.column, v.order))
          .all(&db)
          .await?;
        Ok(resp)
      }

      // Retrieves a list of records for `ids`
      pub async fn list_by_ids<T>(
        db: sea_orm::DatabaseConnection,
        ids: Vec<T>,
      ) -> Result<Vec<$entity::Model>, sea_orm::DbErr>
      where
        T: Into<<$entity::PrimaryKey as sea_orm::PrimaryKeyTrait>::ValueType>,
      {
        let ids: Vec<<$entity::PrimaryKey as sea_orm::PrimaryKeyTrait>::ValueType> =
          ids.into_iter().map(Into::into).collect();
        Self::list_in(db, $entity::Column::Id, ids, None).await
      }
    }
  };
}
