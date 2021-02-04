// use async_trait::async_trait;
//
// use jsonapi::document::{DataDocument, Document, ResourceData};
// use jsonapi::resource::Resource;
// use jsonapi::types::JsonApiId;
//
// use crate::endpoint::manager::Queryset;

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


//
//
// pub struct Endpoint<T, M, S>
// where
//     T: Send + Sync + 'static,
//     M: manager::Manager<T>,
//     S: serializer::Serializer<T = T>,
// {
//     pub manager: M,
//     pub serializer: S,
// }
//
// impl<T, D, M, S> Endpoint<T, M, S>
// where
//     T: Send + Sync + 'static,
//     M: manager::Manager<T>,
//     S: serializer::Serializer<T = T>,
// {
//     pub fn new(dispatcher: D, manager: M, serializer: S) -> Self {
//         Self {
//             manager,
//             serializer,
//         }
//     }
//
//     pub fn default() -> Self {
//         Self {
//             manager: M::default(),
//             serializer: S::default(),
//         }
//     }
//
//     pub async fn retrieve(&self, request: tide::Request<()>, id: JsonApiId) -> tide::Result<tide::Response> {
//         Ok(tide::Response::builder(500).build())
//     }
// }
//
// #[async_trait]
// impl<T, M, S> view::List for Endpoint<T, M, S> {
//     async fn list(&self, mut request: tide::Request<()>) -> tide::Result<tide::Response> {
//         let mut data: Vec<Resource> = Vec::new();
//         let mut included: Vec<Resource> = Vec::new();
//         for object in self.manager.query().all() {
//             let result = match self.serializer.serialize(object) {
//                 Ok(output) => output,
//                 Err(e) => return Ok(tide::Response::builder(500).build()),
//             };
//             data.push(result.data);
//             result.included.map(|i| included.extend(i.into_iter()));
//         }
//         let mut document: DataDocument = Default::default();
//         document.data = Some(ResourceData::Many(data));
//         document.included = Some(included);
//         Ok(tide::Response::builder(200)
//             .body(serde_json::to_string(&document).unwrap())
//             .build())
//     }
// }
//
// #[async_trait]
// impl<T, M, S> view::Create for Endpoint<T, M, S> {
//     async fn create(&self, mut request: tide::Request<()>) -> tide::Result<tide::Response> {
//         unimplemented!();
//     }
// }
//
// impl<T, M, S> view::CollectionView for Endpoint<T, M, S> {
//
// }
