mod handler;
pub mod route;

use self::route::all_routes;
use crate::model::Action;
use std::sync::Arc;
use tokio::sync::{mpsc, oneshot, Mutex};
use warp::hyper::{Body, Response};

pub async fn server_thread(
  mutex: Arc<Mutex<()>>,
  tx: mpsc::Sender<(Action, oneshot::Sender<Response<Body>>)>,
) {
  warp::serve(all_routes(mutex, tx))
    .run(([127, 0, 0, 1], 3030))
    .await;
}
