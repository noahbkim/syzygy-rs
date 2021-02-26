use hyper::Method;
use std::collections::{HashMap, VecDeque};

use crate::request::Action;
use crate::request::Related;
use crate::{Response, View};

type Path = VecDeque<String>;
type HttpRequest = hyper::Request<hyper::Body>;
type HttpResponse = hyper::Response<hyper::Body>;

struct RouterView(Box<dyn View>);

impl RouterView {
    pub async fn item(
        &self,
        request: HttpRequest,
        id: String,
        related: Option<Related>,
    ) -> Response {
        unimplemented!()
    }

    pub async fn collection(&self, request: HttpRequest, related: Option<Related>) -> Response {
        unimplemented!()
    }
}

pub struct RouterLeaf {
    view: RouterView,
}

pub struct RouterNode {
    view: RouterView,
    children: HashMap<String, Router>,
    key: String,
}

pub struct RouterPath {
    children: HashMap<String, Router>,
}

pub enum Router {
    Leaf(RouterLeaf),
    Node(RouterNode),
    Path(RouterPath),
}

impl Router {
    pub async fn route(&self, request: hyper::Request<hyper::Body>) -> Response {
        let mut cursor = self;
        let mut path: Path = Self::split_path(&request);
        let mut related: Option<Related> = None;

        while !path.is_empty() {
            cursor = match cursor {
                Router::Leaf(as_leaf) => {
                    let id = match path.pop_front() {
                        Some(id) => id,
                        None => return as_leaf.view.collection(request, related).await,
                    };
                    return match path.pop_front() {
                        Some(next) => Response::not_found(),
                        None => return as_leaf.view.item(request, id, related).await,
                    };
                }
                Router::Node(as_node) => {
                    let id = match path.pop_front() {
                        Some(id) => id,
                        None => return as_node.view.collection(request, related).await,
                    };
                    match path.pop_front() {
                        Some(next) => {
                            let child = match as_node.children.get(&next) {
                                Some(child) => child,
                                None => return Response::not_found(),
                            };
                            match related.as_mut() {
                                Some(related) => {
                                    related.insert(as_node.key.clone(), id);
                                }
                                None => {
                                    related = {
                                        let mut inner = HashMap::new();
                                        inner.insert(as_node.key.clone(), id);
                                        Some(inner)
                                    };
                                }
                            }
                            child
                        }
                        None => return as_node.view.item(request, id, related).await,
                    }
                }
                Router::Path(as_path) => {
                    let next = match path.pop_front() {
                        Some(next) => next,
                        None => return Response::not_found(),
                    };
                    match as_path.children.get(&next) {
                        Some(child) => child,
                        None => return Response::not_found(),
                    }
                }
            };
        }

        Response::not_found()
    }

    fn split_path(request: &hyper::Request<hyper::Body>) -> VecDeque<String> {
        let mut result = VecDeque::new();
        for part in request.uri().path().trim_matches('/').split('/') {
            result.push_back(part.into());
        }
        result
    }

    fn collection_action(request: &hyper::Request<hyper::Body>) -> Option<Action> {
        match *request.method() {
            Method::GET => Some(Action::List),
            Method::POST => Some(Action::Create),
            _ => None,
        }
    }
}
