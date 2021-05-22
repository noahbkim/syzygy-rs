use crate::resource::{Links, Meta, Resource};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct ErrorSource {
    pub pointer: Option<String>,
    pub parameter: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct Error {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Links>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<ErrorSource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum ResourceData {
    One(Resource),
    Many(Vec<Resource>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pagination {
    pub first: Option<String>,
    pub prev: Option<String>,
    pub next: Option<String>,
    pub last: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Info {
    pub version: Option<String>,
    pub meta: Option<Meta>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct ErrorDocument {
    pub errors: Vec<Error>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Links>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jsonapi: Option<Info>,
}

impl ErrorDocument {
    fn new(errors: Vec<Error>) -> Self {
        Self {
            errors,
            links: None,
            meta: None,
            jsonapi: None,
        }
    }
}

impl Into<Document> for ErrorDocument {
    fn into(self) -> Document {
        Document::Error(self)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DataDocument {
    pub data: ResourceData,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub included: Option<Vec<Resource>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Links>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jsonapi: Option<Info>,
}

impl DataDocument {
    fn new(data: ResourceData) -> Self {
        Self {
            data,
            included: None,
            links: None,
            meta: None,
            jsonapi: None,
        }
    }
}

impl Into<Document> for DataDocument {
    fn into(self) -> Document {
        Document::Data(self)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum Document {
    Error(ErrorDocument),
    Data(DataDocument),
}

impl Document {
    pub fn data(data: ResourceData) -> DataDocument { DataDocument::new(data) }
    pub fn error(errors: Vec<Error>) -> ErrorDocument { ErrorDocument::new(errors) }
}
