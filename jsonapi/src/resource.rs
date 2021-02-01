use crate::types::*;
use std::collections::HashMap;

pub type Meta = HashMap<String, JsonApiValue>;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct LinkObject {
    href: String,
    meta: Meta,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum Link {
    String(String),
    Object(LinkObject),
}

pub type Links = HashMap<String, Link>;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ResourceIdentifier {
    #[serde(rename = "id")]
    pub _id: JsonApiId,
    #[serde(rename = "type")]
    pub _type: JsonApiType,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum ResourceIdentifierData {
    None,
    One(ResourceIdentifier),
    Many(Vec<ResourceIdentifier>),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Relationship {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ResourceIdentifierData>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Links>,
}

pub type Attributes = HashMap<String, JsonApiValue>;
pub type Relationships = HashMap<String, Relationship>;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct Resource {
    #[serde(rename = "id")]
    pub _id: JsonApiId,
    #[serde(rename = "type")]
    pub _type: JsonApiType,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Attributes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationships: Option<Relationships>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Links>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
}

impl Resource {
    pub fn new(id: JsonApiId, type_: JsonApiType) -> Resource {
        Resource {
            _id: id,
            _type: type_,
            attributes: None,
            relationships: None,
            links: None,
            meta: None
        }
    }
}
