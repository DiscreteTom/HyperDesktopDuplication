mod manager;
mod model;
mod server;

use crate::{manager::manager_thread, server::server_thread};
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};

#[tokio::main]
async fn main() {
  let mutex = Arc::new(Mutex::new(()));
  let (tx, rx) = mpsc::channel(1);

  tokio::spawn(async move {
    println!("running on localhost:3030");
    server_thread(mutex, tx).await;
  });

  manager_thread(rx).await;
}
