use async_trait::async_trait;
use hyper::http;
use hyper::{Body, StatusCode};

use crate::view::method::Method;
use crate::view::{Request, Response};
use jsonapi::document::{DataDocument, ResourceData};
use jsonapi::resource::Resource;
use crate::helper::manager::Queryset;

#[async_trait]
pub trait List {
    async fn list(&self, request: Request, parents: Option<Vec<String>>) -> Response {
        http::response::Builder::new()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::empty())
            .unwrap()
    }
}

pub struct NoList;
impl Method for NoList {}
impl List for NoList {}

pub struct ResourceList<T, M, S>
where
    M: crate::helper::manager::Manager<T>,
    S: crate::helper::serializer::Serializer<T = T>,
{
    manager: M,
    serializer: S,
}

impl<T, M, S> Method for ResourceList<T, M, S>
where
    M: crate::helper::manager::Manager<T>,
    S: crate::helper::serializer::Serializer<T = T>,
{
    fn allowed(&self) -> bool {
        return true;
    }
}

impl<T, M, S> List for ResourceList<T, M, S>
where
    M: crate::helper::manager::Manager<T>,
    S: crate::helper::serializer::Serializer<T = T>,
{
    async fn list(&self, request: Request, parents: Option<Vec<String>>) -> Response {
        let mut data: Vec<Resource> = Vec::new();
        let mut included: Vec<Resource> = Vec::new();
        for object in self.manager.query().all() {
            let result = match self.serializer.serialize(object) {
                Ok(output) => output,
                Err(e) => return Response::new(Body::from("error")),
            };
            data.push(result.data);
            result.included.map(|i| included.extend(i.into_iter()));
        }
        let mut document: DataDocument = Default::default();
        document.data = Some(ResourceData::Many(data));
        document.included = Some(included);
        Response::new(Body::from("list response"))
    }
}
