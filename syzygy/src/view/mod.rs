use async_trait::async_trait;

use crate::{Request, Response};
use std::pin::Pin;
use std::future::Future;

pub type ViewResult = Response;
pub type ViewFuture = Pin<Box<dyn Future<Output = Response> + Send>>;

#[async_trait]
pub trait SimpleView<S>: Send + Sync + 'static
where
    S: ?Sized + Send + Sync,
{
    async fn handle(&self, request: Request, state: Box<S>) -> ViewResult;
}
