use std::sync::Arc;

use hyper::server::Server;
use syzygy::router::simple::{SimpleRouter};
use syzygy::http::{Dispatcher};
use syzygy::{Request, Response};
use syzygy::view::ViewResult;
use syzygy::response::Reaction;
use tokio::main;
use async_trait::async_trait;
use jsonapi::document::{Document, DataDocument, ResourceData};
use jsonapi::resource::Resource;

struct TestView {}

#[async_trait]
impl syzygy::view::SimpleView<()> for TestView {
    async fn handle(self: Arc<Self>, request: Request, state: Box<()>) -> ViewResult {
        Response::new(
            Reaction::Accepted,
            Document::data(ResourceData::One(Resource::new(
                "1".into(),
                "resources".into()
            ))).into()
        )
    }
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let view = TestView {};
    let mut router = syzygy::router::simple::SimpleRouter::new(view);
    let dispatcher = Dispatcher::new(router);
    let address = ([127, 0, 0, 1], 3000).into();
    let server = Server::bind(&address).serve(dispatcher);

    println!("Listening on http://{}", address);
    server.await?;

    Ok(())
}
