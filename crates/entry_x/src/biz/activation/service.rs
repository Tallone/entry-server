use crate::{
  biz::entity::activations::{self, Column},
  gen_crud,
};

gen_crud!(activations, activations::Column::Id);

impl Query {
  pub async fn get_by_uid_lid(
    conn: &DatabaseConnection,
    uid: Uuid,
    license_key: &str,
  ) -> Result<Option<activations::Model>, sea_orm::DbErr> {
    Self::get_select()
      .filter(Column::UserId.eq(uid))
      .filter(Column::LicenseKey.eq(license_key))
      .one(conn)
      .await
  }
}
