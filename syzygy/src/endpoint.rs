pub mod model;

pub mod manager;
pub mod serializer;
pub mod deserializer;
pub mod paginator;
pub mod authenticator;
pub mod dispatcher;
pub mod view;

use tide::http::Method;
use jsonapi::document::{Document, DataDocument, ResourceData};
use jsonapi::resource::Resource;
use async_trait::async_trait;
use jsonapi::types::JsonApiId;
use crate::endpoint::manager::Queryset;


pub struct Endpoint<T, D, M, S>
where
    T: Send + Sync + 'static,
    D: dispatcher::Dispatcher,
    M: manager::Manager<T>,
    S: serializer::Serializer<T = T>,
{
    pub dispatcher: D,
    pub manager: M,
    pub serializer: S,
}

impl<T, D, M, S> Endpoint<T, D, M, S>
where
    T: Send + Sync + 'static,
    M: manager::Manager<T>,
    S: serializer::Serializer<T = T>,
{
    pub fn new(dispatcher: D, manager: M, serializer: S) -> Self {
        Self {
            dispatcher,
            manager,
            serializer,
        }
    }

    pub fn default() -> Self {
        Self {
            dispatcher: D::default(),
            manager: M::default(),
            serializer: S::default(),
        }
    }

    pub async fn retrieve(&self, request: tide::Request<()>, id: JsonApiId) -> tide::Result<tide::Response> {
        Ok(tide::Response::builder(500).build())
    }

    pub async fn list(&self, mut request: tide::Request<()>) -> tide::Result<tide::Response> {
        let mut data: Vec<Resource> = Vec::new();
        let mut included: Vec<Resource> = Vec::new();
        for object in self.manager.query().all() {
            let result = match self.serializer.serialize(object) {
                Ok(output) => output,
                Err(e) => return Ok(tide::Response::builder(500).build()),
            };
            data.push(result.data);
            result.included.map(|i| included.extend(i.into_iter()));
        }
        let mut document: DataDocument = Default::default();
        document.data = Some(ResourceData::Many(data));
        document.included = Some(included);
        Ok(tide::Response::builder(200)
            .body(serde_json::to_string(&document).unwrap())
            .build())
    }
}

#[async_trait]
impl<T, D, M, S> tide::Endpoint<()> for Endpoint<T, D, M, S>
where
    T: Send + Sync + 'static,
    M: manager::Manager<T>,
    S: serializer::Serializer<T = T>,
{
    async fn call(&self, mut request: tide::Request<()>) -> tide::Result<tide::Response> {
        match request.method() {
            Method::Get => self.dispatcher.list(request, self).await,
            Method::Post => {
                let document: Document = request.body_json().await?;
                return Ok(tide::Response::builder(200).build());
            }
            // Method::Put => {}
            // Method::Patch => {}
            // Method::Delete => {}
            _ => Ok(tide::Response::builder(404).build())
        }
    }
}
