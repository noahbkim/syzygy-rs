use tide;
use tide::{Request, StatusCode};
use tide::prelude::*;
use tide::http::{Method, Url, Headers};
use route_recognizer::Params;
use serde_json;
use async_trait::async_trait;
use crate::api::serializer::Serializer;
use crate::api::deserializer::Deserializer;

mod user;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    // app.at("/hello/").post(Endpoint {});
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
