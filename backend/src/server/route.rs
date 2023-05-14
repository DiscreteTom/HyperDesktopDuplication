use std::{convert::Infallible, sync::Arc};

use crate::{
  model::Action,
  server::handler::{handle_create_capturer, handle_get_display, handle_list_displays},
};
use tokio::sync::{mpsc, oneshot, Mutex};
use warp::{
  hyper::{Body, Response},
  Filter,
};

pub fn with_mutex(
  mutex: Arc<Mutex<()>>,
) -> impl Filter<Extract = (Arc<Mutex<()>>,), Error = Infallible> + Clone {
  warp::any().map(move || mutex.clone())
}

pub fn with_sender(
  sender: mpsc::Sender<(Action, oneshot::Sender<Response<Body>>)>,
) -> impl Filter<
  Extract = (mpsc::Sender<(Action, oneshot::Sender<Response<Body>>)>,),
  Error = Infallible,
> + Clone {
  warp::any().map(move || sender.clone())
}

/// GET /displays
pub fn list_displays(
  mutex: Arc<Mutex<()>>,
  sender: mpsc::Sender<(Action, oneshot::Sender<Response<Body>>)>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
  warp::path("displays")
    .and(warp::get())
    .and(with_mutex(mutex))
    .and(with_sender(sender))
    .and_then(handle_list_displays)
}

/// GET /displays/:id
/// id is a u32
pub fn get_display(
  mutex: Arc<Mutex<()>>,
  sender: mpsc::Sender<(Action, oneshot::Sender<Response<Body>>)>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
  warp::path!("displays" / u32)
    .and(warp::get())
    .and(with_mutex(mutex))
    .and(with_sender(sender))
    .and_then(handle_get_display)
}

/// PUT /displays/:id/:name
pub fn create_capturer<'a>(
  mutex: Arc<Mutex<()>>,
  sender: mpsc::Sender<(Action, oneshot::Sender<Response<Body>>)>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
  warp::path!("displays" / u32 / String)
    .and(warp::put())
    .and(with_mutex(mutex))
    .and(with_sender(sender))
    .and_then(handle_create_capturer)
}

pub fn all_routes<'a>(
  mutex: Arc<Mutex<()>>,
  sender: mpsc::Sender<(Action, oneshot::Sender<Response<Body>>)>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
  get_display(mutex.clone(), sender.clone())
    .or(create_capturer(mutex.clone(), sender.clone()))
    .or(list_displays(mutex, sender.clone()))
}
