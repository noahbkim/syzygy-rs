use async_trait::async_trait;
use hyper::{http, StatusCode, Body};

use crate::view::actions::{Create, Delete, List, Retrieve, Update};
use crate::{Request, Response};
use crate::view::resource::Method;

#[derive(Default)]
pub struct Disallowed;

#[async_trait]
impl Create for Disallowed {
    async fn create(&self, request: Request, parents: Option<Vec<String>>) -> Response {
        http::response::Builder::new()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::empty())
            .unwrap()
    }
}

#[async_trait]
impl List for Disallowed {
    async fn list(&self, request: Request, parents: Option<Vec<String>>) -> Response {
        http::response::Builder::new()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::empty())
            .unwrap()
    }
}

#[async_trait]
impl Retrieve for Disallowed {
    async fn retrieve(
        &self,
        request: Request,
        id: String,
        parents: Option<Vec<String>>,
    ) -> Response {
        http::response::Builder::new()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::empty())
            .unwrap()
    }
}

#[async_trait]
impl Update for Disallowed {
    async fn update(&self, request: Request, id: String, parents: Option<Vec<String>>) -> Response {
        http::response::Builder::new()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::empty())
            .unwrap()
    }
}

#[async_trait]
impl Delete for Disallowed {
    async fn delete(&self, request: Request, id: String, parents: Option<Vec<String>>) -> Response {
        http::response::Builder::new()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::empty())
            .unwrap()
    }
}

impl Method for Disallowed {
    fn allowed(&self) -> bool {
        return false;
    }
}
