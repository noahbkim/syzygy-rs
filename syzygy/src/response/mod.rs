mod reaction;

use jsonapi::document::Document;
use std::collections::HashMap;

pub use reaction::Reaction;

pub struct Response {
    reaction: Reaction,
    meta: Option<HashMap<String, String>>,
    body: Option<Document>,
}

impl Response {
    pub fn new(reaction: Reaction) -> Self {
        Self {
            reaction,
            meta: None,
            body: None,
        }
    }

    pub fn meta(mut self, meta: HashMap<String, String>) -> Self {
        self.meta = Some(meta);
        self
    }

    pub fn body(mut self, body: Document) -> Self {
        self.body = Some(body);
        self
    }

    pub fn not_found() -> Self {
        Self::new(Reaction::NotFound)
    }
}
