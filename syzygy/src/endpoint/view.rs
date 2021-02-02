use async_trait::async_trait;

#[async_trait]
pub trait View: Send + Sync + 'static {
    async fn list(&self, mut request: tide::Request<()>) -> tide::Result<tide::Response>;
}
