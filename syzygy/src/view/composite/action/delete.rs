use async_trait::async_trait;

use crate::view::composite::RequestData;
use crate::Response;

#[async_trait]
pub trait Delete {
    async fn delete(&self, id: String, data: RequestData) -> Response;
}
