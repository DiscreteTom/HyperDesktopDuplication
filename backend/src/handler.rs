use std::convert::Infallible;

use crate::{
  manager::Manager,
  model::{DisplayInfo, DisplaysInfo},
};

pub async fn list_displays(manager: Manager) -> Result<impl warp::Reply, Infallible> {
  let manager = manager.lock().await;
  Ok(warp::reply::json(&DisplaysInfo {
    displays: manager
      .contexts
      .iter()
      .map(|ctx| {
        let desc = ctx.desc().unwrap();
        DisplayInfo {
          bottom: desc.DesktopCoordinates.bottom,
          top: desc.DesktopCoordinates.top,
          left: desc.DesktopCoordinates.left,
          right: desc.DesktopCoordinates.right,
          name: String::from_utf16_lossy(&desc.DeviceName),
          rotation: desc.Rotation.0,
        }
      })
      .collect(),
  }))
}
