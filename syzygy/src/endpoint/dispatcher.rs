use jsonapi::types::JsonApiId;
use async_trait::async_trait;
use crate::endpoint::view::View;
use tide::http::Method;

pub trait Dispatcher: Send + Sync + Default + 'static {
    fn list(&self) -> bool { true }
    fn create(&self) -> bool { true }
    fn retrieve(&self) -> bool { true }
    fn update(&self) -> bool { true }
    fn delete(&self) -> bool { true }
}

#[derive(Default)]
pub struct DefaultDispatcher;

impl Dispatcher for DefaultDispatcher {}

#[derive(Default)]
pub struct ConfigurableDispatcher {
    list: bool,
    retrieve: bool,
    create: bool,
    update: bool,
    delete: bool,
}

impl Dispatcher for ConfigurableDispatcher {
    fn list(&self) -> bool { self.list }
    fn create(&self) -> bool { self.retrieve }
    fn retrieve(&self) -> bool { self.create }
    fn update(&self) -> bool { self.update }
    fn delete(&self) -> bool { self.delete }
}
