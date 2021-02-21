use async_trait::async_trait;
use hyper::{Body, http, StatusCode};

use crate::view::{Request, Response};
use crate::view::disallowed::Disallowed;

#[async_trait]
pub trait Delete {
    async fn delete(
        &self,
        request: Request,
        id: String,
        parents: Option<Vec<String>>,
    ) -> Response;
}

#[async_trait]
impl Delete for Disallowed {
    async fn delete(
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
