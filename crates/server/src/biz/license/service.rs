use crate::biz::entity::licenses;
use crate::gen_crud;

gen_crud!(licenses, licenses::Column::Id);

#[cfg(test)]
mod tests {
  use crate::internal::{conf::ApplicationConf, db::DB};

  use super::*;
  use dotenvy::dotenv;
  use log::info;
  use sea_orm::{Order, Set};
  use time::{Duration, OffsetDateTime};
  use uuid::Uuid;

  async fn init() -> DB {
    dotenv().expect(".env file not found");
    let _ = env_logger::builder()
      .filter_level(log::LevelFilter::Debug)
      .is_test(true)
      .try_init();

    let conf = ApplicationConf::from_env();
    DB::new(&conf).await.unwrap()
  }

  #[tokio::test]
  async fn test_crud() {
    let db = init().await;
    let key = Uuid::new_v4().to_string();
    let until = util::current_ms() + Duration::days(365).whole_milliseconds() as u64;
    let v = Mutation::create(
      &db.conn,
      licenses::ActiveModel {
        key: Set(key),
        expired_at: Set(Some(until as i64)),
        ..Default::default()
      },
    )
    .await
    .unwrap();
    assert!(v.id > 0);
    let id = v.id.clone();
    let v = Query::get(&db.conn, licenses::Column::Key, v.key).await.unwrap();
    assert!(v.is_some());
    let v = Query::get_by_id(&db.conn, id).await.unwrap();
    assert!(v.is_some());
    assert_eq!(v.unwrap().id, id);
    let v = Mutation::update(
      &db.conn,
      licenses::ActiveModel {
        id: Set(id),
        ..Default::default()
      },
    )
    .await
    .unwrap();
    let r = Mutation::delete_one(
      &db.conn,
      licenses::ActiveModel {
        id: Set(id),
        ..Default::default()
      },
    )
    .await
    .unwrap();
    assert!(r);
  }

  #[tokio::test]
  async fn test_list_in() {
    let db = init().await;
    let keys = vec![
      "2fdbc79a-5a1d-417f-b75b-9dae543d2165",
      "17a5ff05-5d7c-47ac-b3b7-a26c5124354d",
    ];
    let data = Query::list_in(
      &db.conn,
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

  #[test]
  fn test() {
    println!("{}", Duration::days(1).whole_seconds());
  }
}
