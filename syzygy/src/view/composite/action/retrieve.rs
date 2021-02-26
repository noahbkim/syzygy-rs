use async_trait::async_trait;

use crate::view::composite::RequestData;
use crate::{Response};

#[async_trait]
pub trait Retrieve {
    async fn retrieve(&self, id: String, data: RequestData) -> Response;
}
