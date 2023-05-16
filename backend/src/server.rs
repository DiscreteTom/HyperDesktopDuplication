use crate::model::hdd_server::{Hdd, HddServer};
use crate::model::{
  CreateCaptureReply, CreateCaptureRequest, DeleteCaptureReply, DeleteCaptureRequest,
  GetDisplayReply, GetDisplayRequest, ListDisplaysReply, ListDisplaysRequest, TakeCaptureReply,
  TakeCaptureRequest,
};
use crate::model::{HddReply, HddRequest, RequestSender, ServerMutex};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::sync::oneshot;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

#[derive(Debug)]
pub struct TheHdd {
  mutex: ServerMutex,
  sender: RequestSender,
}

impl TheHdd {
  pub fn new(mutex: ServerMutex, sender: RequestSender) -> Self {
    Self { mutex, sender }
  }
}

#[tonic::async_trait]
impl Hdd for TheHdd {
  async fn list_displays(
    &self,
    _request: Request<ListDisplaysRequest>,
  ) -> Result<Response<ListDisplaysReply>, Status> {
    let _ = self.mutex.lock().await;
    let (tx, rx) = oneshot::channel();
    self
      .sender
      .send((HddRequest::ListDisplays, tx))
      .await
      .unwrap();
    if let HddReply::ListDisplays(displays) = rx.await.unwrap() {
      Ok(Response::new(ListDisplaysReply { infos: displays }))
    } else {
      Err(Status::internal("invalid reply"))
    }
  }

  async fn get_display(
    &self,
    request: Request<GetDisplayRequest>,
  ) -> Result<Response<GetDisplayReply>, Status> {
    let _ = self.mutex.lock().await;
    let (tx, rx) = oneshot::channel();
    let request = request.into_inner();
    self
      .sender
      .send((HddRequest::GetDisplay(request.id), tx))
      .await
      .unwrap();
    if let HddReply::GetDisplay(info) = rx.await.unwrap() {
      Ok(Response::new(GetDisplayReply { info: Some(info) }))
    } else {
      Err(Status::internal("invalid reply"))
    }
  }

  async fn create_capture(
    &self,
    request: Request<CreateCaptureRequest>,
  ) -> Result<Response<CreateCaptureReply>, Status> {
    let _ = self.mutex.lock().await;
    let (tx, rx) = oneshot::channel();
    let request = request.into_inner();
    self
      .sender
      .send((HddRequest::CreateCapture(request.id, request.name), tx))
      .await
      .unwrap();
    if let HddReply::CreateCapture(ok) = rx.await.unwrap() {
      Ok(Response::new(CreateCaptureReply { ok }))
    } else {
      Err(Status::internal("invalid reply"))
    }
  }

  async fn delete_capture(
    &self,
    request: Request<DeleteCaptureRequest>,
  ) -> Result<Response<DeleteCaptureReply>, Status> {
    let _ = self.mutex.lock().await;
    let (tx, rx) = oneshot::channel();
    let request = request.into_inner();
    self
      .sender
      .send((HddRequest::DeleteCapture(request.id), tx))
      .await
      .unwrap();
    if let HddReply::DeleteCapture(ok) = rx.await.unwrap() {
      Ok(Response::new(DeleteCaptureReply { ok }))
    } else {
      Err(Status::internal("invalid reply"))
    }
  }

  async fn take_capture(
    &self,
    request: Request<TakeCaptureRequest>,
  ) -> Result<Response<TakeCaptureReply>, Status> {
    let _ = self.mutex.lock().await;
    let (tx, rx) = oneshot::channel();
    let request = request.into_inner();
    self
      .sender
      .send((HddRequest::TakeCapture(request.id), tx))
      .await
      .unwrap();
    if let HddReply::TakeCapture(ok, update) = rx.await.unwrap() {
      Ok(Response::new(TakeCaptureReply { ok, update }))
    } else {
      Err(Status::internal("invalid reply"))
    }
  }
}

pub async fn server_thread(mutex: ServerMutex, tx: RequestSender, port: u16) {
  let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);
  let the_hdd = TheHdd::new(mutex, tx);

  Server::builder()
    .add_service(HddServer::new(the_hdd))
    .serve(addr)
    .await
    .unwrap();
}
