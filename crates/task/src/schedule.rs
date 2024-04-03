use std::{collections::BinaryHeap, future::Future, pin::Pin};

use log::info;
use tokio::sync::Mutex;

use crate::{cons::TaskDuration, TaskLogic};

/// Represent a scheduled task that will be triggered in future
struct ScheduledTask {
  // The task type
  pub task_duration: TaskDuration,
  // The task name
  pub name: String,
  // Whether log at start and end
  pub log: bool,

  // The task logic
  pub(crate) logic: Box<TaskLogic>,

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
  tasks: Mutex<BinaryHeap<ScheduledTask>>,
}

impl Scheduler {
  pub fn new() -> Self {
    Self {
      tasks: Mutex::new(BinaryHeap::new()),
    }
  }

  pub async fn schedule_task<F>(&self, name: &str, log: bool, dur: TaskDuration, logic: F)
  where
    F: 'static,
    F: Fn() -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> + Send + Sync,
  {
    let n = util::current_ms() + dur.get_duration_ms();
    let mut tasks = self.tasks.lock().await;
    tasks.push(ScheduledTask {
      log,
      task_duration: dur,
      name: name.to_owned(),
      logic: Box::new(logic),
      last_trigger_time: 0,
      next_trigger_time: n,
    });
  }

  pub async fn tick(&self) {
    let mut tasks = self.tasks.lock().await;
    let mut cur = util::current_ms();
    while let Some(mut task) = tasks.pop() {
      if task.next_trigger_time > cur {
        tasks.push(task);
        break;
      }

      if task.log {
        info!("[Scheduler-{}] Start executing.", task.name);
      }
      task.last_trigger_time = cur;

      let task_name = task.name.clone();
      let future = (task.logic)();
      tokio::spawn(async move {
        future.await;
        if task.log {
          info!("[Scheduler-{}] Execution completed.", task_name);
        }
      });

      if let TaskDuration::Repeated(dur) = task.task_duration {
        task.next_trigger_time = cur + dur.as_millis() as u64;
        tasks.push(task);
      }

      cur = 0;
    }
  }
}
