use std::convert::Infallible;

use warp::Filter;

use crate::{handler, manager::Manager};

pub fn with_manager(
  manager: Manager,
) -> impl Filter<Extract = (Manager,), Error = Infallible> + Clone {
  warp::any().map(move || manager.clone())
}

/// GET /displays
pub fn get_displays(
  manager: Manager,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
  warp::path("displays")
    .and(warp::get())
    .and(with_manager(manager))
    .and_then(handler::list_displays)
}
