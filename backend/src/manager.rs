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
      HddRequest::ListDisplays => {
        let mut displays = Vec::new();
        manager.contexts.iter().for_each(|ctx| match ctx.desc() {
          Ok(desc) => displays.push(desc.to_info()),
          Err(err) => println!("ListDisplays: {:?}", err),
        });
        HddReply::ListDisplays(Ok(displays))
      }
      HddRequest::GetDisplay(id) => match manager.contexts.get(id as usize) {
        None => HddReply::GetDisplay(Err("invalid id".to_string())),
        Some(ctx) => match ctx.desc() {
          Ok(desc) => HddReply::GetDisplay(Ok(desc.to_info())),
          Err(err) => HddReply::GetDisplay(Err(err)),
        },
      },
      HddRequest::CreateCapture(id, name) => {
        if capturer_map.contains_key(&id) {
          HddReply::CreateCapture(Err("already exists".to_string()))
        } else {
          match manager.contexts.get(id as usize) {
            None => HddReply::CreateCapture(Err("invalid id".to_string())),
            Some(ctx) => match ctx.shared_capturer(&name) {
              Err(err) => HddReply::CreateCapture(Err(err)),
              Ok(capturer) => {
                capturer_map.insert(id, capturer);
                println!("CreateCapturer: id: {}, name: {}", id, name);
                HddReply::CreateCapture(Ok(()))
              }
            },
          }
        }
      }
      HddRequest::DeleteCapture(id) => {
        capturer_map.remove(&id);
        println!("DeleteCapturer: id: {}", id);
        HddReply::DeleteCapture(Ok(()))
      }
      HddRequest::TakeCapture(id) => match capturer_map.get_mut(&id) {
        None => HddReply::TakeCapture(Err("invalid id".to_string())),
        Some(capturer) => match capturer.capture() {
          Err(err) => HddReply::TakeCapture(Err(err)),
          Ok(capture) => {
            if capture.desktop_updated() {
              HddReply::TakeCapture(Ok(true))
            } else {
              HddReply::TakeCapture(Ok(false))
            }
          }
        },
      },
    };
    tx.send(reply).unwrap();
  }
}
