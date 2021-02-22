use async_trait::async_trait;

use crate::view::{Request, Response};

#[async_trait]
pub trait Create {
    async fn create(&self, request: Request, parents: Option<Vec<String>>) -> Response;
}
