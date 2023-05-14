use super::handler::{
  handle_create_capture, handle_delete_capture, handle_get_display, handle_list_displays,
  handle_take_capturer,
};
use crate::model::{ActionSender, ServerMutex};
use std::convert::Infallible;
use warp::Filter;

pub fn with_mutex(
  mutex: ServerMutex,
) -> impl Filter<Extract = (ServerMutex,), Error = Infallible> + Clone {
  warp::any().map(move || mutex.clone())
}

pub fn with_sender(
  sender: ActionSender,
) -> impl Filter<Extract = (ActionSender,), Error = Infallible> + Clone {
  warp::any().map(move || sender.clone())
}

/// GET /displays
pub fn list_displays(
  mutex: ServerMutex,
  sender: ActionSender,
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
  mutex: ServerMutex,
  sender: ActionSender,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
  warp::path!("displays" / u32)
    .and(warp::get())
    .and(with_mutex(mutex))
    .and(with_sender(sender))
    .and_then(handle_get_display)
}

/// PUT /captures/:id/:name
pub fn create_capture(
  mutex: ServerMutex,
  sender: ActionSender,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
  warp::path!("captures" / u32 / String)
    .and(warp::put())
    .and(with_mutex(mutex))
    .and(with_sender(sender))
    .and_then(handle_create_capture)
}

/// DELETE /captures/:id
pub fn delete_capture(
  mutex: ServerMutex,
  sender: ActionSender,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
  warp::path!("captures" / u32)
    .and(warp::delete())
    .and(with_mutex(mutex))
    .and(with_sender(sender))
    .and_then(handle_delete_capture)
}

/// POST /captures/:id
pub fn take_capture(
  mutex: ServerMutex,
  sender: ActionSender,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
  warp::path!("captures" / u32)
    .and(warp::post())
    .and(with_mutex(mutex))
    .and(with_sender(sender))
    .and_then(handle_take_capturer)
}

pub fn all_routes(
  mutex: ServerMutex,
  sender: ActionSender,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
  get_display(mutex.clone(), sender.clone())
    .or(create_capture(mutex.clone(), sender.clone()))
    .or(delete_capture(mutex.clone(), sender.clone()))
    .or(take_capture(mutex.clone(), sender.clone()))
    .or(list_displays(mutex, sender.clone()))
}
