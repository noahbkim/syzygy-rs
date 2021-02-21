use crate::view::{Request, Response};
use async_trait::async_trait;
use hyper::{http, Body, StatusCode};
use crate::view::actions::Disallowed;

#[async_trait]
pub trait Retrieve {
    async fn retrieve(
        &self,
        request: Request,
        id: String,
        parents: Option<Vec<String>>,
    ) -> Response;
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
