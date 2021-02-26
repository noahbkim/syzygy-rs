pub mod composite;

use async_trait::async_trait;

use crate::{Request, Response};

#[async_trait]
pub trait View {
    fn creates(&self) -> bool;
    fn lists(&self) -> bool;
    fn retrieves(&self) -> bool;
    fn updates(&self) -> bool;
    fn deletes(&self) -> bool;

    // Actually invoke view
    async fn handle(&self, request: Request) -> Response;
}
