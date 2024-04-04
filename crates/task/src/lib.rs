use std::{
  future::Future,
  pin::Pin,
  sync::{Arc, OnceLock},
  time::Duration,
};

use log::info;
use schedule::Scheduler;
use tokio::{sync::broadcast, time::interval};

mod cons;
mod schedule;

pub(crate) type TaskLogic = dyn Fn() -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> + Send + Sync;
/// This is a static instance of `OnceLock` that holds an `Arc<Mutex<Scheduler>>`.
/// `OnceLock` ensures that the initialization of the `Scheduler` happens only once,
/// even if multiple threads try to access it simultaneously.
static INSTANCE: OnceLock<Arc<Scheduler>> = OnceLock::new();

/// This function returns an `Arc<Mutex<Scheduler>>` instance.
fn schedule() -> Arc<Scheduler> {
  INSTANCE
    .get_or_init(|| {
      let ret = Scheduler::new();
      Arc::new(ret)
    })
    .clone()
}

/// Start `Scheduler` scheduling
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
            let inst = schedule();
            inst.tick().await;
        }
      }
    }
  });
}

/// `add_once_task` registers a future `f` with the `Scheduler` to be triggered once
/// after the specified `delay`.
///
/// When `log` is true, it will print start and end log;
///
/// example:
/// ```
/// task::add_once_task("Once_task", true, Duration::from_secs(3), || Box::pin(async {})).await;
///
/// ```
pub async fn add_once_task<F>(name: &str, log: bool, delay: Duration, f: F)
where
  F: 'static,
  F: Fn() -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> + Send + Sync,
{
  let inst = schedule();
  inst
    .schedule_task(name, log, cons::TaskDuration::OneTime(delay), f)
    .await;
}

/// `add_repeated_task` registers a future `f` with the `Scheduler` to be triggered
/// periodically with the specified `dur`.
///
/// When `log` is true, it will print start and end log;
///
/// example:
/// ```
/// task::add_repeated_task("Repeated task 5s", true, Duration::from_secs(5), || Box::pin(async {})).await;
/// ```
pub async fn add_repeated_task<F>(name: &str, log: bool, dur: Duration, f: F)
where
  F: 'static,
  F: Fn() -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> + Send + Sync,
{
  let inst = schedule();
  inst
    .schedule_task(name, log, cons::TaskDuration::Repeated(dur), f)
    .await;
}
