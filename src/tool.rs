use time::{macros::format_description, OffsetDateTime, UtcOffset};

use crate::cons;

/// Get current timestamp in milliseconds
pub fn current_ms() -> u64 {
  let dt = OffsetDateTime::now_utc().to_offset(UtcOffset::from_hms(cons::DEFAULT_TIME_OFFSET, 0, 0).unwrap());
  dt.unix_timestamp_nanos() as u64 / 1_000_000
}

/// Get current formatted datetime
pub fn current_time() -> String {
  let dt = OffsetDateTime::now_utc().to_offset(UtcOffset::from_hms(cons::DEFAULT_TIME_OFFSET, 0, 0).unwrap());
  let fmt = format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");
  dt.format(fmt).unwrap()
}
