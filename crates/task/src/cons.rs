use std::time::Duration;

/// Represents the type of task.
pub enum TaskDuration {
  /// A task that runs only once.
  ///
  /// Add--(dur)-->Run->deactive
  OneTime(Duration),

  /// A task that runs periodically.
  ///
  /// Add--(dur)-->Run_Start-->Run_End--(dur)-->Run Start-->Run_End--...
  Repeated(Duration),
}

impl TaskDuration {
  pub fn get_duration_ms(&self) -> u64 {
    match self {
      TaskDuration::OneTime(v) => v.as_millis() as u64,
      TaskDuration::Repeated(v) => v.as_millis() as u64,
    }
  }
}
