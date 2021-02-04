use async_trait::async_trait;
use super::{Request, Response};

#[async_trait]
pub trait List: Send + Sync + 'static {
    async fn list(&self, mut request: Request) -> Response;
    fn list_allowed(&self) -> bool { true }
}

#[async_trait]
pub trait Create: Send + Sync + 'static {
    async fn create(&self, mut request: Request) -> Response;
    fn create_allowed(&self) -> bool { true }
}

#[async_trait]
pub trait Retrieve: Send + Sync + 'static {
    async fn retrieve(&self, mut request: Request, id: String) -> Response;
    fn retrieve_allowed(&self) -> bool { true }
}

#[async_trait]
pub trait Update: Send + Sync + 'static {
    async fn update(&self, mut request: Request, id: String) -> Response;
    fn update_allowed(&self) -> bool { true }
}

#[async_trait]
pub trait Delete: Send + Sync + 'static {
    async fn delete(&self, mut request: Request, id: String) -> Response;
    fn delete_allowed(&self) -> bool { true }
}

pub trait View: List + Create + Retrieve + Update + Delete {}

//
// pub trait CollectionView: List + Create {
//     async fn options(&self, mut request: tide::Request<()>) -> tide::Result<tide::Response> {
//         unimplemented!();
//     }
// }
//
// pub trait ItemView: Retrieve + Update + Delete {
//     async fn options(&self, mut request: tide::Request<()>) -> tide::Result<tide::Response> {
//         unimplemented!();
//     }
// }
// //
// // #[async_trait]
// // pub trait View: List + Create + Retrieve + Update + Delete {
// //     async fn collection(&self, mut request: tide::Request<()>) -> tide::Result<tide::Response> {
// //         match request.method() {
// //             Method::GET => self.list(request).await,
// //             Method::POST => self.create(request).await,
// //             _ => Ok(tide::Response::builder(404).build())
// //         }
// //     }
// //
// //     async fn item(&self, mut request: tide::Request<()>) -> tide::Result<tide::Response> {
// //         unimplemented!();
// //     }
// //
// //     fn bind(&self, mut route: &mut tide::Route<()>) {
// //         route.all(self.collection);
// //         route.at("/:id");
// //     }
// // }
