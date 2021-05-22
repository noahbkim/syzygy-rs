use std::future::{ready, Future, Ready};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use crate::request::Action;
use crate::{Response, router};
use crate::router::Path;

pub struct Handler {
    router: Arc<dyn router::Root>,
}

fn render(response: Response) -> hyper::Response<hyper::Body> {
    let builder = hyper::Response::builder();
    // builder = builder.header();

    let serialized = serde_json::to_string(&response.body).unwrap();
    builder.body(hyper::Body::from(serialized)).unwrap()
}

impl hyper::service::Service<hyper::Request<hyper::Body>> for Handler {
    type Response = hyper::Response<hyper::Body>;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, request: hyper::Request<hyper::Body>) -> Self::Future {
        let wrapped = crate::Request::new(request.uri().path().to_string(), Action::Create);
        let path = Path::split(&wrapped.path);
        if let Some(route) = self.router.route(path) {
            Box::pin(async move {
                let response = route(wrapped).await;
                Ok(render(response))
            })
        } else {
            let result = Ok(render(self.router.lost(wrapped)));
            Box::pin(ready(result))
        }
    }
}

pub struct Dispatcher {
    router: Arc<dyn router::Root>,
}

impl Dispatcher {
    pub fn new<R: router::Root + 'static>(root: R) -> Self {
        Self { router: Arc::new(root) }
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
