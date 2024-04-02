use std::{
  future::Future,
  sync::{Arc, Mutex, OnceLock},
  time::Duration,
};

use log::info;
use schedule::Scheduler;
use tokio::{sync::broadcast, time::interval};

mod cons;
mod schedule;

/// This is a static instance of `OnceLock` that holds an `Arc<Mutex<Scheduler>>`.
/// `OnceLock` ensures that the initialization of the `Scheduler` happens only once,
/// even if multiple threads try to access it simultaneously.
static INSTANCE: OnceLock<Arc<Mutex<Scheduler>>> = OnceLock::new();

/// This function returns an `Arc<Mutex<Scheduler>>` instance.
fn schedule() -> Arc<Mutex<Scheduler>> {
  INSTANCE
    .get_or_init(|| {
      let ret = Scheduler::new();
      Arc::new(Mutex::new(ret))
    })
    .clone()
}

pub fn start_tick(period: Duration, mut shutdown_channel: broadcast::Receiver<()>) {
  tokio::spawn(async move {
    let mut interval = interval(period);
    loop {
      tokio::select! {
        _ = shutdown_channel.recv() => {
            info!("Scheduler is shutting down...");
            return;
        }
        _ = interval.tick() => {
            let binding = schedule();
            let mut s = binding.lock().unwrap();
            s.tick();
        }
      }
    }
  });
}

/// `add_once_task` registers a future `f` with the `Scheduler` to be triggered once
/// after the specified `delay`.
///
/// `name` is a string identifier for the task.
/// `delay` is the duration after which the task should be triggered.
/// `f` is the future that will be executed when the task is triggered.
///
/// Add--(dur)-->Run->Remove
pub fn add_once_task<Fut>(name: &str, delay: Duration, f: Fut)
where
  Fut: Future<Output = ()> + Send + 'static,
{
  let binding = schedule();
  let mut s = binding.lock().unwrap();
  s.add_task(name, cons::TaskDuration::OneTime(delay), f);
}

/// `add_repeated_task` registers a future `f` with the `Scheduler` to be triggered
/// periodically with the specified `dur`.
///
/// `name` is a string identifier for the task.
/// `delay` is the duration after which the task should be triggered repeatedly.
/// `f` is the future that will be executed when the task is triggered.
///
///  Add--(dur)-->Run_Start-->Run_End--(dur)-->Run Start-->Run_End--...
pub fn add_repeated_task<Fut>(name: &str, dur: Duration, f: Fut)
where
  Fut: Future<Output = ()> + Send + 'static,
{
  let binding = schedule();
  let mut s = binding.lock().unwrap();
  s.add_task(name, cons::TaskDuration::Repeated(dur), f);
}

/// `cancel_task` cancels a previously registered task with the given `name`.
///
/// `name` is the string identifier of the task to be canceled.
pub fn cancel_task(name: &str) {
  let binding = schedule();
  let mut s = binding.lock().unwrap();
  s.cancel_task(name);
}
