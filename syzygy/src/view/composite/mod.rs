use async_trait::async_trait;
use jsonapi::document::Document;
use std::collections::HashMap;

use crate::request::{Action, Options, Related};
use crate::view::{Request, Response, View};
use action::{Create, Delete, List, Retrieve, Update};
use method::Method;

pub mod action;
pub mod disallowed;
pub mod method;

pub struct CompositeView<C, L, R, U, D>
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

impl<C, L, R, U, D> CompositeView<C, L, R, U, D>
where
    C: Create + Method + Send + Sync,
    L: List + Method + Send + Sync,
    R: Retrieve + Method + Send + Sync,
    U: Update + Method + Send + Sync,
    D: Delete + Method + Send + Sync,
{
    pub fn new(create: C, list: L, retrieve: R, update: U, delete: D) -> Self {
        Self {
            create,
            list,
            retrieve,
            update,
            delete,
        }
    }
}

pub struct RequestData {
    pub options: Option<Options>,
    pub related: Option<Related>,
    pub meta: Option<HashMap<String, String>>,
    pub body: Option<Document>,
}

impl RequestData {
    pub fn split(request: Request) -> (Action, RequestData) {
        (
            request.action,
            RequestData {
                options: request.options,
                related: request.related,
                meta: request.meta,
                body: request.body,
            },
        )
    }
}

#[async_trait]
impl<C, L, R, U, D> View for CompositeView<C, L, R, U, D>
where
    C: Create + Method + Send + Sync,
    L: List + Method + Send + Sync,
    R: Retrieve + Method + Send + Sync,
    U: Update + Method + Send + Sync,
    D: Delete + Method + Send + Sync,
{
    fn creates(&self) -> bool {
        self.create.allowed()
    }
    fn lists(&self) -> bool {
        self.list.allowed()
    }
    fn retrieves(&self) -> bool {
        self.retrieve.allowed()
    }
    fn updates(&self) -> bool {
        self.update.allowed()
    }
    fn deletes(&self) -> bool {
        self.delete.allowed()
    }

    async fn handle(&self, request: Request) -> Response {
        let (action, data) = RequestData::split(request);
        match action {
            Action::Create => self.create.create(data).await,
            Action::List => self.list.list(data).await,
            Action::Retrieve(id) => self.retrieve.retrieve(id, data).await,
            Action::Update(id) => self.update.update(id, data).await,
            Action::Delete(id) => self.delete.delete(id, data).await,
        }
    }
}
