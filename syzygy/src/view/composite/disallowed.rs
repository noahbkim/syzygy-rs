use async_trait::async_trait;

use super::action::{Create, Delete, List, Retrieve, Update};
use super::method::Method;
use crate::response::Reaction;
use crate::view::composite::RequestData;
use crate::view::Response;

#[derive(Default)]
pub struct Disallowed;

#[async_trait]
impl Create for Disallowed {
    async fn create(&self, request: RequestData) -> Response {
        Response::new(Reaction::MethodNotAllowed)
    }
}

#[async_trait]
impl List for Disallowed {
    async fn list(&self, request: RequestData) -> Response {
        Response::new(Reaction::MethodNotAllowed)
    }
}

#[async_trait]
impl Retrieve for Disallowed {
    async fn retrieve(&self, id: String, request: RequestData) -> Response {
        Response::new(Reaction::MethodNotAllowed)
    }
}

#[async_trait]
impl Update for Disallowed {
    async fn update(&self, id: String, request: RequestData) -> Response {
        Response::new(Reaction::MethodNotAllowed)
    }
}

#[async_trait]
impl Delete for Disallowed {
    async fn delete(&self, id: String, request: RequestData) -> Response {
        Response::new(Reaction::MethodNotAllowed)
    }
}

impl Method for Disallowed {
    fn allowed(&self) -> bool {
        false
    }
}
