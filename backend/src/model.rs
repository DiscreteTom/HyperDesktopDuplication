use rusty_duplication::utils::Result;
use std::sync::Arc;
use tokio::sync::{mpsc, oneshot, Mutex};
use windows::Win32::Graphics::Dxgi::DXGI_OUTPUT_DESC;

pub type ServerMutex = Arc<Mutex<()>>;
pub type RequestSender = mpsc::Sender<(HddRequest, oneshot::Sender<HddReply>)>;
pub type ReplyReceiver = mpsc::Receiver<(HddRequest, oneshot::Sender<HddReply>)>;

tonic::include_proto!("hdd");

#[derive(Debug)]
pub enum HddRequest {
  ListDisplays,
  GetDisplay(u32),
  CreateCapture(u32, String),
  DeleteCapture(u32),
  TakeCapture(u32),
}

#[derive(Debug)]
pub enum HddReply {
  ListDisplays(Result<Vec<DisplayInfo>>),
  GetDisplay(Result<DisplayInfo>),
  CreateCapture(Result<()>),
  DeleteCapture(Result<()>),
  TakeCapture(Result<bool>),
}

pub trait DxgiOutputDescExt {
  fn to_info(&self) -> DisplayInfo;
}

impl DxgiOutputDescExt for DXGI_OUTPUT_DESC {
  fn to_info(&self) -> DisplayInfo {
    DisplayInfo {
      bottom: self.DesktopCoordinates.bottom,
      top: self.DesktopCoordinates.top,
      left: self.DesktopCoordinates.left,
      right: self.DesktopCoordinates.right,
      name: String::from_utf16_lossy(&self.DeviceName),
      rotation: self.Rotation.0,
    }
  }
}
