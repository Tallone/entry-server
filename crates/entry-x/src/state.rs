use axum::extract::FromRef;

use crate::db::DB;

#[derive(Clone)]
pub struct AppState {
  pub db: DB,
}

impl FromRef<AppState> for DB {
  fn from_ref(input: &AppState) -> Self {
    input.db.clone()
  }
}
