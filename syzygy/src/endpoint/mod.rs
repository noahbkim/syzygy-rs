pub mod model;

pub mod manager;
pub mod serializer;
pub mod deserializer;
pub mod paginator;
pub mod authenticator;
pub mod view;
pub mod request;
pub mod response;

pub use request::Request;
pub use response::Response;
use jsonapi::resource::Resource;
use crate::endpoint::manager::{Manager, Queryset};
use crate::endpoint::serializer::Serializer;
use jsonapi::document::{ResourceData, DataDocument};
use hyper::Body;
use async_trait::async_trait;

pub struct Endpoint<T, M, S>
where
    M: manager::Manager<T>,
    S: serializer::Serializer<T = T>
{
    manager: M,
    serializer: S,
}

impl<T, M, S> Endpoint<T, M, S>
where
    M: manager::Manager<T>,
    S: serializer::Serializer<T = T>
{
    pub fn new() -> Self {
        Self { manager: M::default(), serializer: S::default() }
    }
}

#[async_trait]
impl<T, M, S> view::List for Endpoint<T, M, S>
where
    M: manager::Manager<T>,
    S: serializer::Serializer<T = T>
{
    async fn list(&self, mut request: Request, parents: Option<Vec<String>>) -> Response {
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

#[async_trait]
impl<T, M, S> view::Create for Endpoint<T, M, S>
where
    M: manager::Manager<T>,
    S: serializer::Serializer<T = T>
{
    async fn create(&self, mut request: Request, parents: Option<Vec<String>>) -> Response {
        unimplemented!()
    }
}

#[async_trait]
impl<T, M, S> view::Retrieve for Endpoint<T, M, S>
where
    M: manager::Manager<T>,
    S: serializer::Serializer<T = T>
{
    async fn retrieve(&self, mut request: Request, id: String, parents: Option<Vec<String>>) -> Response {
        unimplemented!()
    }
}

#[async_trait]
impl<T, M, S> view::Update for Endpoint<T, M, S>
where
    M: manager::Manager<T>,
    S: serializer::Serializer<T = T>
{
    async fn update(&self, mut request: Request, id: String, parents: Option<Vec<String>>) -> Response {
        unimplemented!()
    }
}

#[async_trait]
impl<T, M, S> view::Delete for Endpoint<T, M, S>
where
    M: manager::Manager<T>,
    S: serializer::Serializer<T = T>
{
    async fn delete(&self, mut request: Request, id: String, parents: Option<Vec<String>>) -> Response {
        unimplemented!()
    }
}

impl<T, M, S> view::View for Endpoint<T, M, S>
    where
        M: manager::Manager<T>,
        S: serializer::Serializer<T = T>
{

}
