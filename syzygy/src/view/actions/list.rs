use async_trait::async_trait;

use crate::view::{Request, Response};

#[async_trait]
pub trait List {
    async fn list(&self, request: Request, parents: Option<Vec<String>>) -> Response;
}

