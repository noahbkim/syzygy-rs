use syzygy::api::{serializer, deserializer};
use jsonapi::types::{JsonApiType, JsonApiValue};
use jsonapi::resource::{Resource, Attributes};
use std::collections::HashMap;
use std::convert::TryInto;
use std::ops::Try;

const USERS: JsonApiType = "users".to_string();

pub struct User {
    pub id: u64,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

pub struct UserSerializer {}

impl serializer::Serializer<User> for UserSerializer {
    fn serialize(&self, user: User) -> Result<serializer::Output, serializer::Error> {
        let mut resource = Resource::new(user.id.to_string(), USERS);
        let mut attributes = Attributes::new();
        attributes.insert("first_name".into(), JsonApiValue::String(user.first_name));
        attributes.insert("last_name".into(), JsonApiValue::String(user.last_name));
        attributes.insert("email".into(), JsonApiValue::String(user.email));
        resource.attributes = Some(attributes);
        return Ok(serializer::Output::data(resource));
    }
}

impl deserializer::Deserializer<User> for UserSerializer {
    fn deserialize(&self, input: deserializer::Input) -> Result<User, deserializer::Error> {
        let attributes: Attributes = input.data.attributes.ok_or(deserializer::Error {})?;
        Ok(User {
            id: attributes.get("id")
                .ok_or(deserializer::Error {})?
                .as_u64()
                .ok_or(deserializer::Error {})?,
            first_name: attributes.get("first_name")
                .ok_or(deserializer::Error {})?
                .as_str()
                .ok_or(deserializer::Error {})?
                .into(),
            last_name: attributes.get("last_name")
                .ok_or(deserializer::Error {})?
                .as_str()
                .ok_or(deserializer::Error {})?
                .into(),
            email: attributes.get("email")
                .ok_or(deserializer::Error {})?
                .as_str()
                .ok_or(deserializer::Error {})?
                .into(),
            password: attributes.get("password")
                .ok_or(deserializer::Error {})?
                .as_str()
                .ok_or(deserializer::Error {})?
                .into(),
        })
    }
}

// #[async_trait]
// impl tide::Endpoint<()> for Endpoint {
//     async fn call(&self, mut request: tide::Request<()>) -> tide::Result<tide::Response> {
//         match request.method() {
//             Method::Options => {}
//             Method::Get => {}
//             Method::Post => {}
//             Method::Put => {}
//             Method::Patch => {}
//             Method::Delete => {}
//             _ => {}
//         }
//         let document: Document = request.body_json().await?;
//         return Ok(tide::Response::builder(200).build());
//     }
// }
