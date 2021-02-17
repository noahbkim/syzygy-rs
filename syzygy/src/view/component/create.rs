use crate::view::{Request, Response};
use async_trait::async_trait;
use hyper::{http, Body, StatusCode};

#[async_trait]
pub trait Create {
    async fn create(&self, request: Request, parents: Option<Vec<String>>) -> Response {
        http::response::Builder::new()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::empty())
            .unwrap()
    }
}
