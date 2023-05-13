use std::convert::Infallible;

use crate::{
  manager::Manager,
  model::{DisplaysInfo, DxgiOutputDescExt},
};

pub async fn handle_list_displays(manager: Manager) -> Result<impl warp::Reply, Infallible> {
  let manager = manager.lock().await;
  Ok(warp::reply::json(&DisplaysInfo {
    displays: manager
      .contexts
      .iter()
      .map(|ctx| {
        let desc = ctx.desc().unwrap();
        desc.to_info()
      })
      .collect(),
  }))
}

pub async fn handle_get_display(id: u32, manager: Manager) -> Result<impl warp::Reply, Infallible> {
  let manager = manager.lock().await;
  let ctx = manager.contexts.get(id as usize).unwrap();
  let desc = ctx.desc().unwrap();
  Ok(warp::reply::json(&(desc.to_info())))
}
