use std::sync::Arc;

use crate::{Request, Response};
use crate::router::{Root, Route, Path, Router};
use crate::view::{SimpleView, ViewFuture};
use crate::response::Reaction;
use jsonapi::document::Document;

pub struct SimpleRouter<S>
    where
        S: Send + Sync + 'static,
{
    view: Arc<dyn SimpleView<S>>,
}

impl<S> SimpleRouter<S>
    where
        S: Send + Sync + 'static,
{
    pub fn new<V: SimpleView<S>>(view: V) -> Self {
        SimpleRouter {
            view: Arc::new(view),
        }
    }
}

impl<S> Router<S> for SimpleRouter<S>
    where
        S: Send + Sync + 'static
{
    fn prepare(&self, state: Box<S>) -> Box<Route> {
        let view = self.view.clone();
        // Box::new(move |request: Request| view.handle(request, state))
        Box::new(|request: Request| -> ViewFuture {
            Box::pin(async move {
                view.handle(request, state).await
            })
        })
    }
}

impl<S> Root for SimpleRouter<S>
    where
        S: Send + Sync + Default + 'static,
{
    fn route(&self, path: Path) -> Option<Box<Route>> {
        Some(self.prepare(Box::new(S::default())))
    }

    fn lost(&self, request: Request) -> Response {
        Response::new(Reaction::NotFound, Document::error(Vec::new()).into())
    }
}