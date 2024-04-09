#[macro_export]
macro_rules! gen_query {
  ($entity:ident, $id:path) => {
    use sea_orm::{QueryOrder, QueryTrait};
    use $crate::internal::db::ColumnOrder;
    pub(crate) struct Query;

    #[allow(dead_code)]
    impl Query {
      // Convenient way to get `Select`
      fn get_select() -> sea_orm::Select<$entity::Entity> {
        $entity::Entity::find()
      }

      // Retrieves a record from the database based on a specified column and value.
      pub async fn get<V>(
        conn: &sea_orm::DatabaseConnection,
        column: $entity::Column,
        v: V,
      ) -> std::result::Result<Option<$entity::Model>, sea_orm::DbErr>
      where
        V: Into<sea_orm::Value>,
      {
        let resp = Self::get_select().filter(column.eq(v)).one(conn).await?;
        Ok(resp)
      }

      // Retrieves a record from the database by id.
      pub async fn get_by_id<T>(
        conn: &sea_orm::DatabaseConnection,
        id: T,
      ) -> std::result::Result<Option<$entity::Model>, sea_orm::DbErr>
      where
        T: Into<<$entity::PrimaryKey as sea_orm::PrimaryKeyTrait>::ValueType>,
      {
        Self::get(conn, $id, id.into()).await
      }

      // Retrieves a list of records from the database
      // based on `column` and `values`.
      pub async fn list_in<V>(
        conn: &sea_orm::DatabaseConnection,
        column: $entity::Column,
        values: Vec<V>,
        order: Option<ColumnOrder<$entity::Column>>,
      ) -> std::result::Result<Vec<$entity::Model>, sea_orm::DbErr>
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
      ) -> std::result::Result<Vec<$entity::Model>, sea_orm::DbErr>
      where
        T: Into<<$entity::PrimaryKey as sea_orm::PrimaryKeyTrait>::ValueType>,
      {
        let ids: Vec<<$entity::PrimaryKey as sea_orm::PrimaryKeyTrait>::ValueType> =
          ids.into_iter().map(Into::into).collect();
        Self::list_in(conn, $id, ids, None).await
      }
    }
  };
}

#[macro_export]
macro_rules! gen_mutation {
  ($entity:ident) => {
    use sea_orm::prelude::*;
    pub(crate) struct Mutation;

    #[allow(dead_code)]
    impl Mutation {
      pub async fn create<'a, C: ConnectionTrait>(
        conn: &C,
        model: $entity::ActiveModel,
      ) -> std::result::Result<$entity::Model, sea_orm::DbErr> {
        let resp = model.insert(conn).await?;
        Ok(resp)
      }

      pub async fn update<'a, C: ConnectionTrait>(
        conn: &C,
        model: $entity::ActiveModel,
      ) -> std::result::Result<$entity::Model, sea_orm::DbErr> {
        let ret = $entity::Entity::update(model).exec(conn).await?;
        Ok(ret)
      }

      pub async fn update_by_column<'a, C: ConnectionTrait, V>(
        conn: &C,
        column: $entity::Column,
        v: V,
        model: $entity::ActiveModel,
      ) -> std::result::Result<$entity::Model, sea_orm::DbErr>
      where
        V: Into<sea_orm::Value>,
      {
        let ret = $entity::Entity::update(model).filter(column.eq(v)).exec(conn).await?;
        Ok(ret)
      }

      // Deletes a record based on the `model`
      //
      // Returns true if the deletion was successful, false if no records were deleted.
      pub async fn delete_one<'a, C: ConnectionTrait>(
        conn: &C,
        model: $entity::ActiveModel,
      ) -> std::result::Result<bool, sea_orm::DbErr> {
        let ret = $entity::Entity::delete(model).exec(conn).await?;
        Ok(ret.rows_affected > 0)
      }
    }
  };
}

#[macro_export]
macro_rules! gen_crud {
  ($entity:ident, $id:path) => {
    use $crate::{gen_mutation, gen_query};
    gen_query!($entity, $id);
    gen_mutation!($entity);
  };
}
