use async_trait::async_trait;

use crate::view::composite::RequestData;
use crate::Response;

#[async_trait]
pub trait List {
    async fn list(&self, request: RequestData) -> Response;
}
