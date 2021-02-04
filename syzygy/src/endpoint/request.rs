pub struct Request {
    pub request: hyper::Request<hyper::Body>,
    pub parents: Option<Vec<String>>,
}

impl Request {
    pub fn new(request: crate::router::Request) -> Request {
        Request { request: request.request, parents: request.parents }
    }
}
