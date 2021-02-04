pub struct Path {
    parts: Vec<String>
}

impl Path {
    pub fn new(request: &hyper::Request<hyper::Body>) -> Self {
        let mut parts: Vec<String> = request.uri().path().split("/").collect();
        parts.reverse();
        Self { parts }
    }

    pub fn pop(&mut self) -> Option<String> {
        self.parts.pop()
    }

    pub fn peek(&self) -> Option<&String> {
        self.parts.last()
    }

    pub fn len(&self) -> usize {
        self.parts.len()
    }

    pub fn is_empty(&self) -> bool {
        self.parts.is_empty()
    }
}

pub struct Request {
    pub request: hyper::Request<hyper::Body>,
    pub parents: Option<Vec<String>>,
    pub path: Path,
}

impl Request {
    pub fn new(request: hyper::Request<hyper::Body>) -> Request {
        Request { request, parents: None, path: Path::new(&request) }
    }
}
