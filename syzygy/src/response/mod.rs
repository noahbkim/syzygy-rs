mod reaction;

use jsonapi::document::Document;
use std::collections::HashMap;

pub use reaction::Reaction;

pub struct Response {
    pub reaction: Reaction,
    pub meta: Option<HashMap<String, String>>,
    pub body: Document,
}

impl Response {
    pub fn new(reaction: Reaction, document: Document) -> Self {
        Self {
            reaction,
            meta: None,
            body: document,
        }
    }
}
