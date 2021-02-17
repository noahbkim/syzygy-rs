use async_trait::async_trait;

pub use create::Create;
pub use delete::Delete;
pub use list::List;
pub use retrieve::Retrieve;
pub use update::Update;

use crate::{Request, Response};
pub use crate::view::method::Method;
use crate::view::{CollectionView, ItemView};

mod create;
mod delete;
mod list;
mod retrieve;
mod update;

pub struct ComponentView<C, L, R, U, D>
where
    C: Create + Method + Send + Sync,
    L: List + Method + Send + Sync,
    R: Retrieve + Method + Send + Sync,
    U: Update + Method + Send + Sync,
    D: Delete + Method + Send + Sync,
{
    create: C,
    list: L,
    retrieve: R,
    update: U,
    delete: D,
}

#[async_trait]
impl<C, L, R, U, D> CollectionView for ComponentView<C, L, R, U, D>
where
    C: Create + Method + Send + Sync,
    L: List + Method + Send + Sync,
    R: Retrieve + Method + Send + Sync,
    U: Update + Method + Send + Sync,
    D: Delete + Method + Send + Sync,
{
    async fn view(&self, request: Request, parents: Option<Vec<String>>) -> Response {
        match request.method() {
            &hyper::Method::GET => self.list.list(request, parents).await,
            &hyper::Method::POST => self.create.create(request, parents).await,
            _ => Response::new(hyper::Body::from("invalid method"))
        }
    }

    async fn options(&self, request: Request) -> Response {
        unimplemented!()
    }
}

#[async_trait]
impl<C, L, R, U, D> ItemView for ComponentView<C, L, R, U, D>
where
    C: Create + Method + Send + Sync,
    L: List + Method + Send + Sync,
    R: Retrieve + Method + Send + Sync,
    U: Update + Method + Send + Sync,
    D: Delete + Method + Send + Sync,
{
    async fn view(&self, request: Request, id: String, parents: Option<Vec<String>>) -> Response {
        match request.method() {
            &hyper::Method::GET => self.retrieve.retrieve(request, id, parents).await,
            &hyper::Method::PATCH |
            &hyper::Method::PUT => self.update.update(request, id, parents).await,
            &hyper::Method::DELETE => self.delete.delete(request, id, parents).await,
            _ => Response::new(hyper::Body::from("invalid method"))
        }
    }

    async fn options(&self, request: Request) -> Response {
        unimplemented!()
    }
}
