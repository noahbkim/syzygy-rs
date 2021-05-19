use std::sync::Arc;

use hyper::server::Server;
use syzygy::router::simple::{SimpleRouter};
use syzygy::http::{Dispatcher};
use syzygy::{View, Request, Response};
use syzygy::view::ViewResult;
use syzygy::response::Reaction;
use tokio::main;
use async_trait::async_trait;

#[derive(Default)]
struct SimpleState {}
struct SimpleView {}

#[async_trait]
impl View<SimpleState> for SimpleView {
    async fn handle(self: Arc<Self>, request: Request, state: Box<SimpleState>) -> ViewResult {
        Ok(Response::new(Reaction::Accepted))
    }
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let view = SimpleView {};
    let mut router = syzygy::router::simple::SimpleRouter::new(view);
    let dispatcher = Dispatcher::new(router);
    let address = ([127, 0, 0, 1], 3000).into();
    let server = Server::bind(&address).serve(dispatcher);

    println!("Listening on http://{}", address);
    server.await?;

    Ok(())
}
