use std::convert::Infallible;

mod user;

use crate::user::{UserList, UserManager, UserSerializer};
use futures_util::{future, TryFutureExt};
use hyper::server::Server;
use hyper::service::{make_service_fn, service_fn, Service};
use hyper::{Body, Error, Method, Request, Response};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use syzygy::server::router::route::Route;
use syzygy::server::router::{Cursor, Router};
use syzygy::server::view::disallowed::Disallowed;
use syzygy::server::view::resource::actions::ResourceList;
use syzygy::server::view::resource::method;
use tokio::main;

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
        let router = self.router.clone();
        Box::pin(async move {
            let route = router.get(Cursor::new(&req));
            Ok(route.view(req).await)
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
        future::ok(Handler {
            router: self.router.clone(),
        })
    }
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut router = Router::empty();

    router.nest(
        "users".into(),
        Router::view(user::UserView::new(
            Disallowed,
            UserList::new(UserManager::default(), UserSerializer::default()),
            Disallowed,
            Disallowed,
            Disallowed,
        )),
    );

    let dispatcher = Dispatcher {
        router: Arc::new(router),
    };
    let address = ([127, 0, 0, 1], 3000).into();
    let server = Server::bind(&address).serve(dispatcher);

    println!("Listening on http://{}", address);
    server.await?;

    Ok(())
}
