use serde_derive::Serialize;
use std::sync::Arc;
use tokio::sync::{mpsc, oneshot};
use warp::hyper::{Body, Response};
use windows::Win32::Graphics::Dxgi::DXGI_OUTPUT_DESC;

pub type ServerMutex = Arc<tokio::sync::Mutex<()>>;
pub type ActionSender = mpsc::Sender<(Action, oneshot::Sender<Response<Body>>)>;
pub type ActionReceiver = mpsc::Receiver<(Action, oneshot::Sender<Response<Body>>)>;

#[derive(Debug)]
pub enum Action {
  ListDisplays,
  GetDisplay(u32),
  CreateCapturer(u32, String),
}

#[derive(Serialize, Clone, Debug)]
pub struct DisplaysInfo {
  pub displays: Vec<DisplayInfo>,
}

#[derive(Serialize, Clone, Debug)]
pub struct DisplayInfo {
  pub bottom: i32,
  pub top: i32,
  pub left: i32,
  pub right: i32,
  pub name: String,
  pub rotation: i32,
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
