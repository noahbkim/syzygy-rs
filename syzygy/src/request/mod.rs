mod action;
mod options;

pub use action::Action;
pub use options::Options;

use jsonapi::document::Document;
use std::collections::HashMap;

pub type Related = HashMap<String, String>;
pub type Meta = HashMap<String, String>;

pub struct Request {
    pub action: Action,
    pub options: Option<Options>,
    pub related: Option<Related>,
    pub meta: Option<Meta>,
    pub body: Option<Document>,
}

impl Request {
    pub fn new(action: Action) -> Self {
        Self {
            action,
            options: None,
            related: None,
            meta: None,
            body: None,
        }
    }
}
