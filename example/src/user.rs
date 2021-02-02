use jsonapi::resource::{Attributes, Resource};
use jsonapi::types::{JsonApiValue};
use syzygy::endpoint::{deserializer, manager, serializer, Endpoint};

const USERS: &str = "users";

pub struct User {
    pub id: u64,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Default)]
pub struct UserSerializer;

impl serializer::Serializer for UserSerializer {
    type T = User;

    fn serialize(&self, user: &User) -> Result<serializer::Output, serializer::Error> {
        let mut resource = Resource::new(user.id.to_string(), USERS.into());
        let mut attributes = Attributes::new();
        attributes.insert(
            "first_name".into(),
            JsonApiValue::String(user.first_name.clone()),
        );
        attributes.insert(
            "last_name".into(),
            JsonApiValue::String(user.last_name.clone()),
        );
        attributes.insert("email".into(), JsonApiValue::String(user.email.clone()));
        resource.attributes = Some(attributes);
        return Ok(serializer::Output::data(resource));
    }
}

#[derive(Default)]
pub struct UserDeserializer;

impl deserializer::Deserializer for UserDeserializer {
    type T = User;

    fn deserialize(&self, input: deserializer::Input) -> Result<User, deserializer::Error> {
        let attributes: Attributes = input.data.attributes.ok_or(deserializer::Error {})?;
        Ok(User {
            id: attributes
                .get("id")
                .ok_or(deserializer::Error {})?
                .as_u64()
                .ok_or(deserializer::Error {})?,
            first_name: attributes
                .get("first_name")
                .ok_or(deserializer::Error {})?
                .as_str()
                .ok_or(deserializer::Error {})?
                .into(),
            last_name: attributes
                .get("last_name")
                .ok_or(deserializer::Error {})?
                .as_str()
                .ok_or(deserializer::Error {})?
                .into(),
            email: attributes
                .get("email")
                .ok_or(deserializer::Error {})?
                .as_str()
                .ok_or(deserializer::Error {})?
                .into(),
            password: attributes
                .get("password")
                .ok_or(deserializer::Error {})?
                .as_str()
                .ok_or(deserializer::Error {})?
                .into(),
        })
    }
}

pub struct UserQueryset {
    data: Vec<User>,
}

impl manager::Queryset for UserQueryset {
    type T = User;

    fn first(&self) -> Option<&User> {
        self.data.first()
    }

    fn all(&self) -> &Vec<User> {
        &self.data
    }

    fn filter(self, key: &str, value: &str) -> Self {
        self
    }

    fn order(self, key: &str, value: &str) -> Self {
        self
    }
}

#[derive(Default)]
pub struct UserManager;

impl manager::Manager for UserManager {
    type T = User;
    type Queryset = UserQueryset;

    fn query(&self) -> UserQueryset {
        UserQueryset {
            data: vec![User {
                id: 1,
                first_name: "Noah".to_string(),
                last_name: "Kim".to_string(),
                email: "noahbkim@gmail.com".to_string(),
                password: "asdfjkl;".to_string(),
            }],
        }
    }

    fn create(&self, resource: Resource) -> User {
        User {
            id: 1,
            first_name: "Noah".to_string(),
            last_name: "Kim".to_string(),
            email: "noahbkim@gmail.com".to_string(),
            password: "asdfjkl;".to_string(),
        }
    }

    fn update(&self, resource: Resource) -> User {
        User {
            id: 1,
            first_name: "Noah".to_string(),
            last_name: "Kim".to_string(),
            email: "noahbkim@gmail.com".to_string(),
            password: "asdfjkl;".to_string(),
        }
    }

    fn delete(&self, resource: Resource) -> User {
        User {
            id: 1,
            first_name: "Noah".to_string(),
            last_name: "Kim".to_string(),
            email: "noahbkim@gmail.com".to_string(),
            password: "asdfjkl;".to_string(),
        }
    }
}

pub type UserEndpoint = Endpoint<User, UserSerializer, UserDeserializer, UserManager>;
