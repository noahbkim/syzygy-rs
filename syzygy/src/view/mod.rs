use async_trait::async_trait;

use crate::{Request, Response};
use crate::view::dispatcher::Dispatcher;

mod extra;
mod method;
mod component;

#[async_trait]
pub trait CollectionView {
    async fn view(&self, request: Request, parents: Option<Vec<String>>) -> Response;
    async fn options(&self, request: Request) -> Response;
}

pub trait ItemView {
    async fn view(&self, request: Request, id: String, parents: Option<Vec<String>>) -> Response;
    async fn options(&self, request: Request) -> Response;
}

// TODO: trait alias once stabilized
pub trait View: CollectionView + ItemView {}
impl<T> View for T where T: CollectionView + ItemView {}
