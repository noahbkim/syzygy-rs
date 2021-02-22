use async_trait::async_trait;

pub use create::Create;
pub use delete::Delete;
pub use list::List;
pub use retrieve::Retrieve;
pub use update::Update;

use crate::view::{CollectionView, ItemView};
use crate::{Request, Response};

pub mod create;
pub mod delete;
pub mod list;
pub mod retrieve;
pub mod update;

pub struct Disallowed;

#[async_trait]
pub trait CollectionActions {
    async fn create(&self, request: Request, parents: Option<Vec<String>>) -> Response;
    async fn list(&self, request: Request, parents: Option<Vec<String>>) -> Response;
    fn allowed(&self) -> (bool, bool);
}

#[async_trait]
pub trait ItemActions {
    async fn retrieve(&self, request: Request, id: String, parents: Option<Vec<String>>) -> Response;
    async fn update(&self, request: Request, id: String, parents: Option<Vec<String>>) -> Response;
    async fn delete(&self, request: Request, id: String, parents: Option<Vec<String>>) -> Response;
    fn allowed(&self) -> (bool, bool, bool);
}

#[async_trait]
impl<T> CollectionView for T
where
    T: CollectionActions + Send + Sync,
{
    async fn view(&self, request: Request, parents: Option<Vec<String>>) -> Response {
        match request.method() {
            &hyper::Method::GET => self.list(request, parents).await,
            &hyper::Method::POST => self.create(request, parents).await,
            _ => Response::new(hyper::Body::from("invalid method")),
        }
    }

    async fn options(&self, request: Request) -> Response {
        unimplemented!()
    }
}

#[async_trait]
impl<T> ItemView for T
where
    T: ItemActions + Send + Sync,
{
    async fn view(&self, request: Request, id: String, parents: Option<Vec<String>>) -> Response {
        match request.method() {
            &hyper::Method::GET => self.retrieve(request, id, parents).await,
            &hyper::Method::PATCH | &hyper::Method::PUT => self.update(request, id, parents).await,
            &hyper::Method::DELETE => self.delete(request, id, parents).await,
            _ => Response::new(hyper::Body::from("invalid method")),
        }
    }

    async fn options(&self, request: Request) -> Response {
        unimplemented!()
    }
}
