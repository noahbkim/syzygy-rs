use tide;
use tide::{Request, StatusCode};
use tide::prelude::*;

mod user;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/users/").all(user::UserEndpoint::new(
        user::UserSerializer {},
        user::UserDeserializer {},
        user::UserManager {}));
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
