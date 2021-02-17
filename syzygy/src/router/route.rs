use async_trait::async_trait;

use crate::view::{View, ItemView, CollectionView};
use crate::{Request, Response};
use hyper::StatusCode;

pub enum Route<'a> {
    Collection(&'a dyn View, Option<Vec<String>>),
    Item(&'a dyn View, String, Option<Vec<String>>),
    None,
}

#[async_trait]
impl Route {
    pub async fn view(mut self, mut request: Request) -> Response {
        match self {
            Route::Collection(view, parents) => CollectionView::view(view, request, parents),
            Route::Item(view, id, parents) => ItemView::view(view, request, id, parents),
            Route::None => hyper::http::response::Builder::new()
                .status(StatusCode::NOT_FOUND)
                .body(hyper::Body::empty())
                .unwrap()
        }
    }
}
