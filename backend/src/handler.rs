use std::convert::Infallible;

use crate::{
  manager::Manager,
  model::{DisplaysInfo, DxgiOutputDescExt},
};

pub async fn list_displays(manager: Manager) -> Result<impl warp::Reply, Infallible> {
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
