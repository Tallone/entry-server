use sea_orm::{
  ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, Order, PrimaryKeyTrait, QueryFilter,
  QueryOrder, QueryTrait, Select, Value,
};

use crate::db::ColumnOrder;
use crate::domain::entity::licenses;
use crate::domain::Result;

pub(crate) struct Mutation;
pub(crate) struct Query;

impl Mutation {
  pub async fn create(db: DatabaseConnection, model: licenses::ActiveModel) -> Result<licenses::Model> {
    let resp = model.insert(&db).await?;
    Ok(resp)
  }

  pub async fn update<V>(
    db: DatabaseConnection,
    column: licenses::Column,
    value: V,
    model: licenses::ActiveModel,
  ) -> Result<licenses::Model>
  where
    V: Into<Value>,
  {
    let ret = licenses::Entity::update(model)
      .filter(column.eq(value))
      .exec(&db)
      .await?;
    Ok(ret)
  }

  // Deletes a record based on the `model`
  //
  // Returns true if the deletion was successful, false if no records were deleted.
  pub async fn delete_one<V>(db: DatabaseConnection, model: licenses::ActiveModel) -> Result<bool> {
    let ret = licenses::Entity::delete(model).exec(&db).await?;
    Ok(ret.rows_affected > 0)
  }
}

impl Query {
  // Convenient way to get `Select`
  fn get_select() -> Select<licenses::Entity> {
    licenses::Entity::find()
  }

  // Retrieves a record from the database based on a specified column and value.
  pub async fn get<V>(db: DatabaseConnection, column: licenses::Column, v: V) -> Result<Option<licenses::Model>>
  where
    V: Into<Value>,
  {
    let resp = Query::get_select().filter(column.eq(v)).one(&db).await?;
    Ok(resp)
  }

  // Retrieves a record from the database by id.
  pub async fn get_by_id<T>(db: DatabaseConnection, id: T) -> Result<Option<licenses::Model>>
  where
    T: Into<<licenses::PrimaryKey as PrimaryKeyTrait>::ValueType>,
  {
    Query::get(db, licenses::Column::Id, id.into()).await
  }

  // Retrieves a list of records from the database
  // based on `column` and `values`.
  pub async fn list_in<V>(
    db: DatabaseConnection,
    column: licenses::Column,
    values: Vec<V>,
    order: Option<ColumnOrder<licenses::Column>>,
  ) -> Result<Vec<licenses::Model>>
  where
    V: Into<Value>,
  {
    let mut filters = Condition::all();
    filters = filters.add(column.is_in(values));
    let resp = licenses::Entity::find()
      .filter(filters)
      .apply_if(order, |q, v| q.order_by(v.column, v.order))
      .all(&db)
      .await?;
    Ok(resp)
  }

  // Retrieves a list of records for `ids`
  pub async fn list_by_ids<T>(db: DatabaseConnection, ids: Vec<T>) -> Result<Vec<licenses::Model>>
  where
    T: Into<<licenses::PrimaryKey as PrimaryKeyTrait>::ValueType>,
  {
    let ids: Vec<<licenses::PrimaryKey as PrimaryKeyTrait>::ValueType> = ids.into_iter().map(Into::into).collect();
    Query::list_in(db, licenses::Column::Id, ids, None).await
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use dotenvy::dotenv;
  use log::info;
  use sea_orm::Set;
  use time::{Duration, OffsetDateTime};
  use uuid::Uuid;

  use crate::{
    conf::ApplicationConf,
    db::{ColumnOrder, DB},
  };

  async fn init() -> DB {
    dotenv().expect(".env file not found");
    let _ = env_logger::builder()
      .filter_level(log::LevelFilter::Info)
      .is_test(true)
      .try_init();

    let conf = ApplicationConf::from_env();
    DB::new(&conf).await.unwrap()
  }

  #[tokio::test]
  async fn test_create() {
    let db = init().await;
    let key = Uuid::new_v4().to_string();
    let until = OffsetDateTime::now_utc().checked_add(Duration::days(365)).unwrap();
    Mutation::create(
      db.conn,
      licenses::ActiveModel {
        key: Set(key),
        status: Set(0),
        valid_until: Set(until),
        ..Default::default()
      },
    )
    .await
    .unwrap();
  }

  #[tokio::test]
  async fn test_list_in() {
    let db = init().await;
    let keys = vec![
      "2fdbc79a-5a1d-417f-b75b-9dae543d2165",
      "17a5ff05-5d7c-47ac-b3b7-a26c5124354d",
    ];
    let data = Query::list_in(
      db.conn,
      licenses::Column::Key,
      keys,
      Some(ColumnOrder {
        column: licenses::Column::CreatedAt,
        order: Order::Asc,
      }),
    )
    .await
    .unwrap();
    info!("data: {:?}", data);
  }
}
