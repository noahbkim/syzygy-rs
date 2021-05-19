use std::sync::Arc;

use crate::View;
use crate::router::{Root, Node, wrap, Route, Path};
use std::collections::HashMap;

pub struct SimpleRouter<S>
    where
        S: Send + Sync + 'static,
{
    view: Arc<dyn View<S>>,
    children: HashMap<String, Box<dyn Node<S>>>,
}

impl<S> SimpleRouter<S>
    where
        S: Send + Sync + Default + 'static,
{
    pub fn new<V: View<S>>(view: V) -> Self {
        SimpleRouter {
            view: Arc::new(view),
            children: HashMap::new()
        }
    }
}

impl<S> Root for SimpleRouter<S>
    where
        S: Send + Sync + Default + 'static,
{
    fn route(&self, path: Path) -> Option<Box<Route>> {
        Some(wrap(self.view.clone(), Box::new(S::default())))
    }
}