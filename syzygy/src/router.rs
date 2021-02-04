use std::collections::HashMap;

use hyper;

pub mod request;
pub mod response;
pub use request::Request;
pub use response::Response;
use crate::endpoint;
use crate::endpoint::view::View;
use hyper::{Method, Body};

pub struct Router {
    children: HashMap<String, Router>,
    view: Option<Box<dyn View>>,
}

impl Router {
    pub fn view<V: View>(view: V) -> Self {
        Self {
            children: HashMap::new(),
            view: Some(Box::new(view)),
        }
    }

    pub fn empty() -> Self {
        Self {
            children: HashMap::new(),
            view: None
        }
    }

    pub fn nest(&mut self, path: String, router: Router) {
        self.children.insert(path, router);
    }

    pub async fn route(&self, request: Request) -> Response {
        match &self.view {
            Some(view) => self.route_view(view, request).await,
            None => self.route_through(request).await,
        }
    }

    async fn route_view(&self, view: &Box<dyn View>, mut request: Request) -> Response {
        match request.path.pop() {
            Some(id) => match request.path.pop() {
                Some(next) => self.route_view_nested(id, next, request).await,
                None => self.route_view_item(view, id, request).await,
            },
            None => self.route_view_collection(view, request).await,
        }
    }

    async fn route_view_collection(&self, view: &Box<dyn View>, request: Request) -> Response {
        match request.request.method() {
            &Method::GET => view.list(endpoint::Request::new(request)).await,
            &Method::POST => view.create(endpoint::Request::new(request)).await,
            _ => Response::new(Body::from("invalid method")),
        }
    }

    async fn route_view_item(&self, view: &Box<dyn View>, id: String, request: Request) -> Response {
        match request.request.method() {
            &Method::GET => view.retrieve(endpoint::Request::new(request), id).await,
            &Method::PUT | &Method::PATCH => view.update(endpoint::Request::new(request), id).await,
            &Method::DELETE => view.delete(endpoint::Request::new(request), id).await,
            _ => Response::new(Body::from("invalid method")),
        }
    }

    async fn route_view_nested(&self, id: String, next: String, mut request: Request) -> Response {
        match self.children.get(&next) {
            Some(router) => {
                if request.parents.is_none() {
                    request.parents = Some(vec![id]);
                }
                router.route(request).await
            },
            None => Response::new(Body::from("no nested")),
        }
    }

    async fn route_through(&self, mut request: Request) -> Response {
        match request.path.pop() {
            Some(next) => match self.children.get(&next) {
                Some(router) => router.route(request).await,
                None => Response::new(Body::from("no child")),
            },
            None => Response::new(Body::from("no view"))
        }
    }
}
