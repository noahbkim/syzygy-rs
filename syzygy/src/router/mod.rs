use std::collections::HashMap;

pub mod cursor;
pub mod route;
pub use cursor::Cursor;

use crate::router::route::Route;
use crate::view::View;

pub struct Router {
    children: HashMap<String, Router>,
    view: Option<Box<dyn View>>,
}

impl Router {
    pub fn view<V: View + 'static>(view: V) -> Self {
        Self {
            children: HashMap::new(),
            view: Some(Box::new(view)),
        }
    }

    pub fn empty() -> Self {
        Self {
            children: HashMap::new(),
            view: None,
        }
    }

    pub fn nest(&mut self, path: String, router: Router) {
        self.children.insert(path, router);
    }

    pub fn get(&self, mut cursor: Cursor) -> Route {
        let view = match &self.view {
            Some(view) => view,
            None => return self.get_through(cursor),
        };
        let id = match cursor.path.pop() {
            Some(id) => id,
            None => return Route::Collection(&**view, cursor.parents),
        };
        match cursor.path.pop() {
            Some(next) => self.get_nested(id, next, cursor),
            None => Route::Item(&**view, id, cursor.parents),
        }
    }

    fn get_nested(&self, id: String, next: String, mut cursor: Cursor) -> Route {
        let router = match self.children.get(&next) {
            Some(router) => router,
            None => return Route::None,
        };
        match cursor.parents.as_mut() {
            Some(parents) => parents.push(id),
            None => cursor.parents = Some(vec![id]),
        }
        router.get(cursor)
    }

    fn get_through(&self, mut cursor: Cursor) -> Route {
        let next = match cursor.path.pop() {
            Some(next) => next,
            None => return Route::None,
        };
        match self.children.get(&next) {
            Some(router) => router.get(cursor),
            None => Route::None,
        }
    }
}
