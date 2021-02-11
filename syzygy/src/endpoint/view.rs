use async_trait::async_trait;
use super::{Request, Response};

#[async_trait]
pub trait List: Send + Sync {
    async fn list(&self, mut request: Request, parents: Option<Vec<String>>) -> Response;
    fn list_allowed(&self) -> bool { true }
}

#[async_trait]
pub trait Create: Send + Sync {
    async fn create(&self, mut request: Request, parents: Option<Vec<String>>) -> Response;
    fn create_allowed(&self) -> bool { true }
}

#[async_trait]
pub trait Retrieve: Send + Sync {
    async fn retrieve(&self, mut request: Request, id: String, parents: Option<Vec<String>>) -> Response;
    fn retrieve_allowed(&self) -> bool { true }
}

#[async_trait]
pub trait Update: Send + Sync {
    async fn update(&self, mut request: Request, id: String, parents: Option<Vec<String>>) -> Response;
    fn update_allowed(&self) -> bool { true }
}

#[async_trait]
pub trait Delete: Send + Sync {
    async fn delete(&self, mut request: Request, id: String, parents: Option<Vec<String>>) -> Response;
    fn delete_allowed(&self) -> bool { true }
}

#[async_trait]
pub trait View: List + Create + Retrieve + Update + Delete {
    async fn options(&self, mut request: Request) -> Response {
        unimplemented!();
    }
}
