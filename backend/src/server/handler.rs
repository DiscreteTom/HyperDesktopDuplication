use crate::model::{Action, ActionSender, ServerMutex};
use std::convert::Infallible;
use tokio::sync::oneshot;

pub async fn handle_list_displays(
  mutex: ServerMutex,
  sender: ActionSender,
) -> Result<impl warp::Reply, Infallible> {
  let _ = mutex.lock().await;
  let (tx, rx) = oneshot::channel();
  sender.send((Action::ListDisplays, tx)).await.unwrap();
  Ok(rx.await.unwrap())
}

pub async fn handle_get_display(
  id: u32,
  mutex: ServerMutex,
  sender: ActionSender,
) -> Result<impl warp::Reply, Infallible> {
  let _ = mutex.lock().await;
  let (tx, rx) = oneshot::channel();
  sender.send((Action::GetDisplay(id), tx)).await.unwrap();
  Ok(rx.await.unwrap())
}

pub async fn handle_create_capturer(
  id: u32,
  name: String,
  mutex: ServerMutex,
  sender: ActionSender,
) -> Result<impl warp::Reply, Infallible> {
  let _ = mutex.lock().await;
  let (tx, rx) = oneshot::channel();
  sender
    .send((Action::CreateCapturer(id, name), tx))
    .await
    .unwrap();
  Ok(rx.await.unwrap())
}

pub async fn handle_delete_capturer(
  id: u32,
  mutex: ServerMutex,
  sender: ActionSender,
) -> Result<impl warp::Reply, Infallible> {
  let _ = mutex.lock().await;
  let (tx, rx) = oneshot::channel();
  sender.send((Action::DeleteCapturer(id), tx)).await.unwrap();
  Ok(rx.await.unwrap())
}
