mod handler;
mod manager;
mod model;
mod route;

use crate::route::all_routes;

#[tokio::main]
async fn main() {
  let manager = manager::init_manager();

  println!("running on localhost:3030");

  warp::serve(all_routes(manager))
    .run(([127, 0, 0, 1], 3030))
    .await;
}
