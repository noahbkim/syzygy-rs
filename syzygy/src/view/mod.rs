use async_trait::async_trait;

use crate::{Error, Request, Response};
use std::sync::Arc;

pub type ViewResult = Result<Response, Box<dyn Error>>;

#[async_trait]
pub trait View<S>: Send + Sync + 'static
where
    S: ?Sized + Send + Sync,
{
    async fn handle(self: Arc<Self>, request: Request, state: Box<S>) -> ViewResult;
}
