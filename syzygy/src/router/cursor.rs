use std::fmt;

pub struct Path {
    parts: Vec<String>,
}

impl Path {
    pub fn new(request: &hyper::Request<hyper::Body>) -> Self {
        let mut parts: Vec<String> = Vec::new();

        for part in request.uri().path().trim_matches('/').split('/') {
            parts.push(part.to_string());
        }
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

pub struct Cursor {
    pub parents: Option<Vec<String>>,
    pub path: Path,
}

impl Cursor {
    pub fn new(request: &hyper::Request<hyper::Body>) -> Cursor {
        Cursor {
            parents: None,
            path: Path::new(&request),
        }
    }
}
