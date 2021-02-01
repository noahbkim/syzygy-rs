use crate::resource::{Links, Meta, Resource};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum ResourceData {
    None,
    Single(Resource),
    Multiple(Vec<Resource>),
}

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
pub struct DocumentError {
    pub errors: Vec<Error>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Links>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jsonapi: Option<Info>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct DocumentData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ResourceData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub included: Option<Vec<Resource>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Links>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jsonapi: Option<Info>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum Document {
    Error(DocumentError),
    Data(DocumentData),
}
