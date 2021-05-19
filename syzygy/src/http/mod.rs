use std::future::{ready, Future, Ready};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use crate::error::{Error, RouterError};
use crate::request::Action;
use crate::response::Reaction;
use crate::{Request, Response, router};
use crate::router::Path;

pub struct Handler {
    router: Arc<dyn router::Root>,
}

impl hyper::service::Service<hyper::Request<hyper::Body>> for Handler {
    type Response = hyper::Response<hyper::Body>;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, request: hyper::Request<hyper::Body>) -> Self::Future {
        let path_string = request.uri().path().to_string();
        let path = Path::split(&path_string);
        if let Some(route) = self.router.route(path) {
            let wrapped = crate::Request::new(path_string, Action::Create);
            Box::pin(async move {
                let response = route(wrapped).await;
                Ok(Self::Response::new(hyper::Body::from("Hello World")))
            })
        } else {
            panic!();
            // Box::pin(ready(Err(Box::new(RouterError {}) as Box<dyn Error>)))
        }
    }
}

pub struct Dispatcher {
    router: Arc<dyn router::Root>,
}

impl Dispatcher {
    pub fn new<R: router::Root + 'static>(root: R) -> Self {
        return Self { router: Arc::new(root) }
    }
}

impl<T> hyper::service::Service<T> for Dispatcher {
    type Response = Handler;
    type Error = std::io::Error;
    type Future = Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Ok(()).into()
    }

    fn call(&mut self, _: T) -> Self::Future {
        let handler = Handler {
            router: self.router.clone(),
        };
        ready(Ok(handler))
    }
}
