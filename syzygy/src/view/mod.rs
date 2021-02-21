use async_trait::async_trait;

use crate::{Request, Response};

mod actions;
mod resource;
mod disallowed;

#[async_trait]
pub trait CollectionView {
    async fn view(&self, request: Request, parents: Option<Vec<String>>) -> Response;
    async fn options(&self, request: Request) -> Response;
}

#[async_trait]
pub trait ItemView {
    async fn view(&self, request: Request, id: String, parents: Option<Vec<String>>) -> Response;
    async fn options(&self, request: Request) -> Response;
}

// TODO: trait alias once stabilized
pub trait View: CollectionView + ItemView {}
impl<T> View for T where T: CollectionView + ItemView {}
