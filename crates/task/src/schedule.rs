use std::{collections::BinaryHeap, future::Future, pin::Pin, time::Duration};

use log::info;
use tokio::{
  sync::broadcast,
  time::{interval, Interval},
};

use crate::cons::TaskDuration;

/// Represent a scheduled task that will be triggered in future
struct ScheduledTask {
  // The task type
  pub task_duration: TaskDuration,
  // The task name
  pub name: String,

  // The task logic
  pub(crate) logic: Pin<Box<dyn Future<Output = ()> + Send>>,

  // Last trigger time in epoch milliseconds
  pub(crate) last_trigger_time: u64,

  // Next trigger time in epoch milliseconds
  pub(crate) next_trigger_time: u64,
}

impl PartialEq for ScheduledTask {
  fn eq(&self, other: &Self) -> bool {
    self.name == other.name
  }
}

impl Eq for ScheduledTask {}

impl PartialOrd for ScheduledTask {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

// Implement ordering for tasks based on their execution time
impl Ord for ScheduledTask {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self.next_trigger_time.cmp(&other.next_trigger_time).reverse() // reverse to make it a min-heap
  }
}

pub(crate) struct Scheduler {
  tasks: BinaryHeap<ScheduledTask>,
}

impl Scheduler {
  pub fn new() -> Self {
    Self {
      tasks: BinaryHeap::new(),
    }
  }

  pub fn add_task<Fut>(&mut self, name: &str, dur: TaskDuration, logic: Fut)
  where
    Fut: Future<Output = ()> + Send + 'static,
  {
    let n = util::current_ms() + dur.get_duration_ms();
    let task = ScheduledTask {
      task_duration: dur,
      name: name.to_owned(),
      logic: Box::pin(logic),
      last_trigger_time: 0,
      next_trigger_time: n,
    };
    self.tasks.push(task);
  }

  pub fn cancel_task(&mut self, task_name: &str) {
    self.tasks = self
      .tasks
      .drain()
      .filter(|task| task.name.as_str() != task_name)
      .collect();
  }

  pub fn tick(&mut self) {
    let mut cur = util::current_ms();
    while cur != 0 {
      if let Some(mut task) = self.tasks.peek_mut() {
        if task.next_trigger_time <= cur {
          task.last_trigger_time = cur;

          if let TaskDuration::OneTime(_) = task.task_duration {
            self.cancel_task(&task.name);
          }
          tokio::spawn(async move {
            task.logic.await;
            task.next_trigger_time = cur + task.task_duration.get_duration_ms();
          });
        }
      }

      cur = 0;
    }
  }
}
