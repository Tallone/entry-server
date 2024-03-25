use crate::{domain::entity::activations, gen_crud};

gen_crud!(activations);

#[cfg(test)]
mod tests {
  use dotenvy::dotenv;
use log::info;
use sea_orm::Set;
  use super::*;

  use crate::{conf::ApplicationConf, db::DB};
  use time::OffsetDateTime;

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
    let model = Mutation::create(db.conn, activations::ActiveModel {
        license_id: Set(1),
        user_id: Set(Uuid::new_v4()),
        ip_address: Set("127.0.0.1".to_owned()),
        activation_date: Set(OffsetDateTime::now_utc()),
        ..Default::default()
    }).await.unwrap();
    info!("mode: {:?}", model);
  }
}
