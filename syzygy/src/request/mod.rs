mod action;
mod options;

use jsonapi::document::Document;
use std::collections::HashMap;

pub use action::Action;
pub use options::Options;

pub type Meta = HashMap<String, String>;

pub struct Request {
    pub path: String,
    pub action: Action,
    pub options: Option<Options>,
    pub meta: Option<Meta>,
    pub body: Option<Document>,
}

impl Request {
    pub fn new(path: String, action: Action) -> Self {
        Self {
            path,
            action,
            options: None,
            meta: None,
            body: None,
        }
    }
}
