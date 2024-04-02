use std::{future::Future, time::Duration};

use log::info;
use tokio::{
  sync::broadcast,
  time::{interval, Interval},
};

use crate::cons::TaskType;

/// Represent a scheduled task that will be triggered in future
pub struct ScheduledTask<F> {
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

impl<F> ScheduledTask<F>
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

pub struct Scheduler {
  pub period: Duration,
  pub shutdown_channel: broadcast::Receiver<()>,
}

impl Scheduler {
  pub fn new(period: Duration, shutdown_channel: broadcast::Receiver<()>) -> Self {
    Self {
      period,
      shutdown_channel,
    }
  }

  pub fn start_tick(&mut self) {
    let mut this = self;
    tokio::spawn(async {
      let mut tick_interval = interval(this.period);

      loop {
        tokio::select! {
            _ = this.shutdown_channel.recv() => {
                break;
            }

            _ = tick_interval.tick() => {
                    info!("Scheduler running")
                }
        }
      }
    });
  }
}
