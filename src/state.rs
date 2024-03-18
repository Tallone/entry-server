use crate::db::DB;

#[derive(Clone)]
pub struct AppState {
  pub db: DB,
}
