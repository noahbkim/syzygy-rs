use jsonapi::types::JsonApiId;
use async_trait::async_trait;
use crate::endpoint::view::View;

// #[async_trait]
// pub trait Retrieve: Send + Sync + 'static {
//     async fn retrieve(&self, mut request: tide::Request<()>, id: JsonApiId) -> tide::Result<tide::Response>;
// }
//
// #[async_trait]
// pub trait List: Send + Sync + 'static {
//     async fn list(&self, mut request: tide::Request<()>) -> tide::Result<tide::Response>;
// }

#[async_trait]
pub trait Dispatcher<V>: Send + Sync + Default + 'static
where
    V: crate::endpoint::view::View
{
    async fn list(&self, mut request: tide::Request<()>, handler: &V) -> tide::Result<tide::Response> {
        handler.list(request).await
    }
}
