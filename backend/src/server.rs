mod handler;
pub mod route;

use self::route::all_routes;
use crate::model::{ActionSender, ServerMutex};

pub async fn server_thread(mutex: ServerMutex, tx: ActionSender, port: u16) {
  warp::serve(all_routes(mutex, tx))
    .run(([127, 0, 0, 1], port))
    .await;
}
