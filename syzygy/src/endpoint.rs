pub mod model;

pub mod manager;
pub mod serializer;
pub mod deserializer;
pub mod paginator;
pub mod authenticator;

use tide::http::Method;
use jsonapi::document::{Document, DataDocument, ResourceData};
use jsonapi::resource::Resource;
use async_trait::async_trait;
use std::marker::PhantomData;
use jsonapi::resource::ResourceIdentifierData::Many;
use crate::endpoint::manager::Queryset;

pub struct Endpoint<T, S, D, M>
where
    T: Send + Sync + 'static,
    S: serializer::Serializer<T>,
    D: deserializer::Deserializer<T>,
    M: manager::Manager<T>,
{
    pub serializer: S,
    pub deserializer: D,
    pub manager: M,
    pub t: PhantomData<T>,
}

impl<T, S, D, M> Endpoint<T, S, D, M>
where
    T: Send + Sync + 'static,
    S: serializer::Serializer<T>,
    D: deserializer::Deserializer<T>,
    M: manager::Manager<T>,
{
    pub fn new(serializer: S, deserializer: D, manager: M) -> Self {
        Self {
            serializer,
            deserializer,
            manager,
            t: PhantomData::default(),
        }
    }

    pub fn default() -> Self {
        Self {
            serializer: S::default(),
            deserializer: D::default(),
            manager: M::default(),
            t: PhantomData::default(),
        }
    }
}

#[async_trait]
impl<T, S, D, M> tide::Endpoint<()> for Endpoint<T, S, D, M>
where
    T: Send + Sync + 'static,
    S: serializer::Serializer<T>,
    D: deserializer::Deserializer<T>,
    M: manager::Manager<T>,
{
    async fn call(&self, mut request: tide::Request<()>) -> tide::Result<tide::Response> {
        match request.method() {
            Method::Get => {
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
