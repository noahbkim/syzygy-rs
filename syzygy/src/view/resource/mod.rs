use async_trait::async_trait;

use crate::view::actions::{Create, List, Retrieve, Update, Delete, CollectionActions, ItemActions};
use crate::{Request, Response};

pub mod actions;
pub mod method;
pub mod peripheral;

pub use method::Method;

pub struct ResourceView<C, L, R, U, D>
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

impl<C, L, R, U, D> ResourceView<C, L, R, U, D>
where
    C: Create + Method + Send + Sync,
    L: List + Method + Send + Sync,
    R: Retrieve + Method + Send + Sync,
    U: Update + Method + Send + Sync,
    D: Delete + Method + Send + Sync,
{
    pub fn new(create: C, list: L, retrieve: R, update: U, delete: D) -> Self {
        Self { create, list, retrieve, update, delete }
    }
}

#[async_trait]
impl<C, L, R, U, D> CollectionActions for ResourceView<C, L, R, U, D>
where
    C: Create + Method + Send + Sync,
    L: List + Method + Send + Sync,
    R: Retrieve + Method + Send + Sync,
    U: Update + Method + Send + Sync,
    D: Delete + Method + Send + Sync,
{
    async fn create(&self, request: Request, parents: Option<Vec<String>>) -> Response {
        self.create.create(request, parents).await
    }

    async fn list(&self, request: Request, parents: Option<Vec<String>>) -> Response {
        self.list.list(request, parents).await
    }

    fn allowed(&self) -> (bool, bool) {
        (self.create.allowed(), self.list.allowed())
    }
}

#[async_trait]
impl<C, L, R, U, D> ItemActions for ResourceView<C, L, R, U, D>
where
    C: Create + Method + Send + Sync,
    L: List + Method + Send + Sync,
    R: Retrieve + Method + Send + Sync,
    U: Update + Method + Send + Sync,
    D: Delete + Method + Send + Sync,
{
    async fn retrieve(&self, request: Request, id: String, parents: Option<Vec<String>>) -> Response {
        self.retrieve.retrieve(request, id, parents).await
    }

    async fn update(&self, request: Request, id: String, parents: Option<Vec<String>>) -> Response {
        self.update.update(request, id, parents).await
    }

    async fn delete(&self, request: Request, id: String, parents: Option<Vec<String>>) -> Response {
        self.delete.delete(request, id, parents).await
    }

    fn allowed(&self) -> (bool, bool, bool) {
        (self.retrieve.allowed(), self.update.allowed(), self.delete.allowed())
    }
}
