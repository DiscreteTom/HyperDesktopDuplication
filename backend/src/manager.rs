use crate::model::{DxgiOutputDescExt, HddReply, HddRequest, ReplyReceiver};
use rusty_duplication::{
  capturer::{model::Capturer, shared::SharedCapturer},
  manager::Manager,
  utils::FrameInfoExt,
};
use std::collections::HashMap;

pub async fn manager_thread(mut rx: ReplyReceiver) {
  let manager = Manager::default().unwrap();
  let mut capturer_map: HashMap<u32, SharedCapturer> = HashMap::new();

  loop {
    let (req, tx) = rx.recv().await.unwrap();
    let reply = match req {
      HddRequest::ListDisplays => HddReply::ListDisplays(
        manager
          .contexts
          .iter()
          .map(|ctx| ctx.desc().unwrap().to_info())
          .collect(),
      ),
      HddRequest::GetDisplay(id) => HddReply::GetDisplay(
        manager
          .contexts
          .get(id as usize)
          .unwrap()
          .desc()
          .unwrap()
          .to_info(),
      ),
      HddRequest::CreateCapture(id, name) => {
        println!("CreateCapturer: id: {}, name: {}", id, name);

        capturer_map.insert(
          id,
          manager
            .contexts
            .get(id as usize)
            .unwrap()
            .shared_capturer(&name)
            .unwrap(),
        );
        HddReply::CreateCapture
      }
      HddRequest::DeleteCapture(id) => {
        capturer_map.remove(&id);
        println!("DeleteCapturer: id: {}", id);
        HddReply::DeleteCapture
      }
      HddRequest::TakeCapture(id) => {
        if capturer_map
          .get_mut(&id)
          .unwrap()
          .capture()
          .unwrap()
          .desktop_updated()
        {
          HddReply::TakeCapture(true)
        } else {
          HddReply::TakeCapture(false)
        }
      }
    };
    tx.send(reply).unwrap();
  }
}
