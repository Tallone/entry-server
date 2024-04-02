use std::time::Duration;

/// Represents the type of task.
pub enum TaskType {
  /// A task that runs only once.
  ///
  /// Add--(dur)-->Run->deactive
  OneTime(Duration),

  /// A task that runs periodically.
  ///
  /// Add--(dur)-->Run_Start-->Run_End--(dur)-->Run Start-->Run_End--...
  Periodic(Duration),
}

impl TaskType {
  pub fn get_duration_ms(&self) -> u64 {
    match self {
      TaskType::OneTime(v) => v.as_millis() as u64,
      TaskType::Periodic(v) => v.as_millis() as u64,
    }
  }
}
