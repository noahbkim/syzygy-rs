use tide;
use tide::{Request, StatusCode};
use tide::prelude::*;
use crate::user::UserEndpoint;

mod user;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/users/").all(UserEndpoint::default());
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
