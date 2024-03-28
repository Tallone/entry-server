#[macro_export]
macro_rules! gen_crud {
  ($entity:ident) => {
    use crate::db::ColumnOrder;
    use sea_orm::prelude::*;
    use sea_orm::{QueryOrder, QueryTrait};

    #[allow(dead_code)]
    pub(crate) mod mutation {
      use super::*;

      pub async fn create(
        conn: &sea_orm::DatabaseConnection,
        model: $entity::ActiveModel,
      ) -> Result<$entity::Model, sea_orm::DbErr> {
        let resp = model.insert(conn).await?;
        Ok(resp)
      }

      pub async fn update(
        conn: &sea_orm::DatabaseConnection,
        model: $entity::ActiveModel,
      ) -> Result<$entity::Model, sea_orm::DbErr> {
        let ret = $entity::Entity::update(model).exec(conn).await?;
        Ok(ret)
      }

      // Deletes a record based on the `model`
      //
      // Returns true if the deletion was successful, false if no records were deleted.
      pub async fn delete_one(
        conn: &sea_orm::DatabaseConnection,
        model: $entity::ActiveModel,
      ) -> Result<bool, sea_orm::DbErr> {
        let ret = $entity::Entity::delete(model).exec(conn).await?;
        Ok(ret.rows_affected > 0)
      }
    }

    #[allow(dead_code)]
    pub(crate) mod query {
      use super::*;

      // Convenient way to get `Select`
      fn get_select() -> sea_orm::Select<$entity::Entity> {
        $entity::Entity::find()
      }

      // Retrieves a record from the database based on a specified column and value.
      pub async fn get<V>(
        conn: &sea_orm::DatabaseConnection,
        column: $entity::Column,
        v: V,
      ) -> Result<Option<$entity::Model>, sea_orm::DbErr>
      where
        V: Into<sea_orm::Value>,
      {
        let resp = get_select().filter(column.eq(v)).one(conn).await?;
        Ok(resp)
      }

      // Retrieves a record from the database by id.
      pub async fn get_by_id<T>(
        conn: &sea_orm::DatabaseConnection,
        id: T,
      ) -> Result<Option<$entity::Model>, sea_orm::DbErr>
      where
        T: Into<<$entity::PrimaryKey as sea_orm::PrimaryKeyTrait>::ValueType>,
      {
        get(conn, $entity::Column::Id, id.into()).await
      }

      // Retrieves a list of records from the database
      // based on `column` and `values`.
      pub async fn list_in<V>(
        conn: &sea_orm::DatabaseConnection,
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
          .all(conn)
          .await?;
        Ok(resp)
      }

      // Retrieves a list of records for `ids`
      pub async fn list_by_ids<T>(
        conn: &sea_orm::DatabaseConnection,
        ids: Vec<T>,
      ) -> Result<Vec<$entity::Model>, sea_orm::DbErr>
      where
        T: Into<<$entity::PrimaryKey as sea_orm::PrimaryKeyTrait>::ValueType>,
      {
        let ids: Vec<<$entity::PrimaryKey as sea_orm::PrimaryKeyTrait>::ValueType> =
          ids.into_iter().map(Into::into).collect();
        list_in(conn, $entity::Column::Id, ids, None).await
      }
    }
  };
}
