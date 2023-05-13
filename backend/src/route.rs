use std::convert::Infallible;

use warp::Filter;

use crate::{
  handler::{handle_create_capturer, handle_get_display, handle_list_displays},
  manager::Manager,
};

pub fn with_manager(
  manager: Manager,
) -> impl Filter<Extract = (Manager,), Error = Infallible> + Clone {
  warp::any().map(move || manager.clone())
}

/// GET /displays
pub fn list_displays(
  manager: Manager,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
  warp::path("displays")
    .and(warp::get())
    .and(with_manager(manager))
    .and_then(handle_list_displays)
}

/// GET /displays/:id
/// id is a u32
pub fn get_display(
  manager: Manager,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
  warp::path!("displays" / u32)
    .and(warp::get())
    .and(with_manager(manager))
    .and_then(handle_get_display)
}

/// PUT /displays/:id/:name
pub fn create_capturer(
  manager: Manager,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
  warp::path!("displays" / u32 / String)
    .and(warp::put())
    .and(with_manager(manager))
    .and_then(handle_create_capturer)
}

pub fn all_routes(
  manager: Manager,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
  get_display(manager.clone())
    .or(create_capturer(manager.clone()))
    .or(list_displays(manager))
}
