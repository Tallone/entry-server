use crate::{
  domain::entity::activations::{self, Column},
  gen_crud,
};

gen_crud!(activations);

impl Query {
  pub async fn get_by_uid_lid(
    conn: &DatabaseConnection,
    uid: Uuid,
    license_id: &str,
  ) -> Result<Option<activations::Model>, sea_orm::DbErr> {
    Self::get_select()
      .filter(Column::UserId.eq(uid))
      .filter(Column::LicenseId.eq(license_id))
      .one(conn)
      .await
  }
}
