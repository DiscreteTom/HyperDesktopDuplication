use std::{convert::Infallible, sync::Arc};

use tokio::sync::{mpsc, oneshot, Mutex};
use warp::hyper::{Body, Response};

use crate::model::Action;

pub async fn handle_list_displays(
  mutex: Arc<Mutex<()>>,
  sender: mpsc::Sender<(Action, oneshot::Sender<Response<Body>>)>,
) -> Result<impl warp::Reply, Infallible> {
  let _ = mutex.lock().await;
  let (tx, rx) = oneshot::channel();
  sender.send((Action::ListDisplays, tx)).await.unwrap();
  Ok(rx.await.unwrap())
}

pub async fn handle_get_display(
  id: u32,
  mutex: Arc<Mutex<()>>,
  sender: mpsc::Sender<(Action, oneshot::Sender<Response<Body>>)>,
) -> Result<impl warp::Reply, Infallible> {
  let _ = mutex.lock().await;
  let (tx, rx) = oneshot::channel();
  sender.send((Action::GetDisplay(id), tx)).await.unwrap();
  Ok(rx.await.unwrap())
}

pub async fn handle_create_capturer(
  id: u32,
  name: String,
  mutex: Arc<Mutex<()>>,
  sender: mpsc::Sender<(Action, oneshot::Sender<Response<Body>>)>,
) -> Result<impl warp::Reply, Infallible> {
  let _ = mutex.lock().await;
  let (tx, rx) = oneshot::channel();
  sender
    .send((Action::CreateCapturer(id, name), tx))
    .await
    .unwrap();
  Ok(rx.await.unwrap())
}
