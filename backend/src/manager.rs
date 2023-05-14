use crate::model::{Action, ActionReceiver, DisplaysInfo, DxgiOutputDescExt};
use rusty_duplication::{capturer::shared::SharedCapturer, manager::Manager};
use std::collections::HashMap;
use warp::Reply;

pub async fn manager_thread(mut rx: ActionReceiver) {
  let manager = Manager::default().unwrap();
  let mut capturer_map: HashMap<u32, SharedCapturer> = HashMap::new();

  loop {
    let (action, tx) = rx.recv().await.unwrap();
    let result = match action {
      Action::ListDisplays => warp::reply::json(&DisplaysInfo {
        displays: manager
          .contexts
          .iter()
          .map(|ctx| ctx.desc().unwrap().to_info())
          .collect(),
      })
      .into_response(),
      Action::GetDisplay(id) => warp::reply::json(
        &manager
          .contexts
          .get(id as usize)
          .unwrap()
          .desc()
          .unwrap()
          .to_info(),
      )
      .into_response(),
      Action::CreateCapturer(id, name) => {
        capturer_map.insert(
          id,
          manager
            .contexts
            .get(id as usize)
            .unwrap()
            .shared_capturer(&name)
            .unwrap(),
        );
        warp::reply::json(&"ok").into_response()
      }
    };
    tx.send(result).unwrap();
  }
}
