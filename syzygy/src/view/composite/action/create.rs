use async_trait::async_trait;

use crate::view::composite::RequestData;
use crate::Response;

#[async_trait]
pub trait Create {
    async fn create(&self, request: RequestData) -> Response;
}
