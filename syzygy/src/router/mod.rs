pub mod simple;

use std::any::Any;
use std::collections::{HashMap, VecDeque};
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::sync::Arc;

use crate::view::ViewResult;
use crate::{Request, View};

pub type ViewFuture = Pin<Box<dyn Future<Output = ViewResult> + Send>>;
pub type Route = dyn FnOnce(Request) -> ViewFuture + Send;

pub struct Path<'a> {
    components: VecDeque<&'a str>,
}

impl<'a> Path<'a> {
    pub fn split(string: &'a String) -> Self {
        Self {
            components: {
                let mut result = VecDeque::new();
                for part in string.trim_matches('/').split('/') {
                    result.push_back(part.into());
                }
                result
            }
        }
    }
}

pub trait Root: Send + Sync {
    fn route(&self, path: Path) -> Option<Box<Route>>;
}

pub trait Node<S>: Send + Sync
where
    S: Send + Sync + 'static
{
    fn route(&self, path: Path, state: Box<S>) -> Option<Box<Route>>;
}

pub fn wrap<S>(view: Arc<dyn View<S>>, state: Box<S>) -> Box<Route>
where
    S: Send + Sync + 'static,
{
    Box::new(move |request: Request| view.handle(request, state))
}



// pub struct RouterLeaf {
//     view: Box<dyn View>,
// }
//
// pub struct RouterNode {
//     view: Box<dyn View>,
//     children: HashMap<String, Router>,
//     key: String,
// }
//
// pub struct RouterPath {
//     children: HashMap<String, Router>,
// }
//
// pub enum Router {
//     Leaf(RouterLeaf),
//     Node(RouterNode),
//     Path(RouterPath),
// }
//

// pub struct Route<S>
// where
//     S: ?Sized + Send + Sync
// {
//     pub state: Box<S>,
//     pub view: Arc<dyn View<S>>,
// }
//

//
// impl Router {
//     pub fn route(&self, url: &str) -> Route {
//         let mut cursor = self;
//         let mut path: Path = Self::split_path(url);
//         let mut related: Option<Related> = None;
//
//         while !path.is_empty() {
//             cursor = match cursor {
//                 Router::Leaf(as_leaf) => {
//                     let id = match path.pop_front() {
//                         Some(id) => id,
//                         None => return Route::new(related, &as_leaf.view),
//                     };
//                     return match path.pop_front() {
//                         Some(next) => Route::None,
//                         None => return Route::Item(id, related, &as_leaf.view),
//                     };
//                 }
//                 Router::Node(as_node) => {
//                     let id = match path.pop_front() {
//                         Some(id) => id,
//                         None => return Route::Collection(related, &as_node.view),
//                     };
//                     match path.pop_front() {
//                         Some(next) => {
//                             let child = match as_node.children.get(&next) {
//                                 Some(child) => child,
//                                 None => return Route::None,
//                             };
//                             match related.as_mut() {
//                                 Some(related) => {
//                                     related.insert(as_node.key.clone(), id);
//                                 }
//                                 None => {
//                                     related = {
//                                         let mut inner = HashMap::new();
//                                         inner.insert(as_node.key.clone(), id);
//                                         Some(inner)
//                                     };
//                                 }
//                             }
//                             child
//                         }
//                         None => return Route::Item(id, related, &as_node.view),
//                     }
//                 }
//                 Router::Path(as_path) => {
//                     let next = match path.pop_front() {
//                         Some(next) => next,
//                         None => return Route::None,
//                     };
//                     match as_path.children.get(&next) {
//                         Some(child) => child,
//                         None => return Route::None,
//                     }
//                 }
//             };
//         }
//
//         Route::None
//     }
//
//     fn split_path(path: &str) -> VecDeque<String> {
//         let mut result = VecDeque::new();
//         for part in path.trim_matches('/').split('/') {
//             result.push_back(part.into());
//         }
//         result
//     }
// }
