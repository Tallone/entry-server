use std::future::Future;

use crate::cons::TaskType;

/// Represent a task that will trigger in future
pub struct Task<F> {
  // The task type
  pub task_type: TaskType,
  // The task name
  pub name: String,

  // The task logic
  pub(crate) func: F,

  // Last trigger time in epoch milliseconds
  pub(crate) last_trigger_time: u64,

  // Next trigger time in epoch milliseconds
  pub(crate) next_trigger_time: u64,
}

impl<F> Task<F>
where
  F: Future + Send + 'static,
  F::Output: Send + 'static,
{
  pub fn new(name: &str, task_type: TaskType, func: F) -> Self {
    let next_trigger_time = util::current_ms() + task_type.get_duration_ms();
    Self {
      name: name.to_owned(),
      task_type,
      func,
      next_trigger_time,
      last_trigger_time: 0,
    }
  }
}
