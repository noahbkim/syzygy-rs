use std::convert::Infallible;

mod user;

use hyper::service::{make_service_fn, service_fn, Service};
use hyper::{Body, Request, Response, Error, Method};
use hyper::server::Server;
use std::sync::Arc;
use syzygy::router::{Router, Cursor};
use std::task::{Context, Poll};
use std::pin::Pin;
use futures_util::{future, TryFutureExt};
use tokio::main;
use syzygy::router::route::Route;
use std::future::Future;

struct Handler {
    router: Arc<Router>,
}

impl Service<Request<Body>> for Handler {
    type Response = Response<Body>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let route = self.router.get(Cursor::new(&req));
        Box::pin(async move {
            Ok(match route {
                Route::Item(view, id, parents) => {
                    match req.method() {
                        &Method::GET => view.retrieve(req, id, parents).await,
                        &Method::PATCH |
                        &Method::PUT => view.update(req, id, parents).await,
                        &Method::DELETE => view.delete(req, id, parents).await,
                        _ => Response::new(Body::from("invalid method"))
                    }
                },
                Route::Collection(view, parents) => {
                    match req.method() {
                        &Method::GET => view.list(req, parents).await,
                        &Method::POST => view.create(req, parents).await,
                        _ => Response::new(Body::from("invalid method"))
                    }
                },
                Route::None => Response::new(Body::from("not found"))
            })
        })
    }
}

struct Dispatcher {
    router: Arc<Router>,
}

impl<T> Service<T> for Dispatcher {
    type Response = Handler;
    type Error = std::io::Error;
    type Future = future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Ok(()).into()
    }

    fn call(&mut self, _: T) -> Self::Future {
        future::ok(Handler { router: self.router.clone() })
    }
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut router = Router::empty();
    router.nest("users".into(), Router::view(user::UserEndpoint::new()));

    let dispatcher = Dispatcher { router: Arc::new(router) };
    let address = ([127, 0, 0, 1], 3000).into();
    let server = Server::bind(&address).serve(dispatcher);

    println!("Listening on http://{}", address);
    server.await?;

    Ok(())
}
