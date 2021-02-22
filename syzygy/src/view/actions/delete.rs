use async_trait::async_trait;

use crate::view::{Request, Response};

#[async_trait]
pub trait Delete {
    async fn delete(&self, request: Request, id: String, parents: Option<Vec<String>>) -> Response;
}
