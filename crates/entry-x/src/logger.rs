use env_logger::Env;
use std::io::Write;

use crate::tool;

pub fn init() {
  let mut builder = env_logger::Builder::from_env(Env::default().default_filter_or("info"));
  builder
    .format(|buf, record| {
      writeln!(
        buf,
        "[{} {} {}] {}",
        record.level(),
        tool::current_time(),
        record.module_path().unwrap_or_default(),
        record.args()
      )
    })
    .init();
}
