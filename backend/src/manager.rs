use std::sync::Arc;

use tokio::sync::Mutex;

pub type Manager = Arc<Mutex<rusty_duplication::manager::Manager>>;

pub fn init_manager() -> Manager {
  Arc::new(Mutex::new(
    rusty_duplication::manager::Manager::default().unwrap(),
  ))
}
