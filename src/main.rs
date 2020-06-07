use std::sync::mpsc;

mod settings;
mod worker_thread;
mod slack_publisher;

fn main() {
  let (tx, rx) = mpsc::channel();

  let settings = settings::get_settings();

  for setting in settings {
    let trx = tx.clone();

    worker_thread::get_worker_thread(trx, setting)
  }

  loop {
    for msg in &rx {
      slack_publisher::notify_slack(msg);
    }
  }
}
