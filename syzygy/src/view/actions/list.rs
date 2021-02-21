use async_trait::async_trait;
use hyper::{Body, StatusCode};
use hyper::http;

use crate::view::{Request, Response};
use crate::view::disallowed::Disallowed;

#[async_trait]
pub trait List {
    async fn list(&self, request: Request, parents: Option<Vec<String>>) -> Response;
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
