use anyhow::Result;

#[async_trait::async_trait]
pub trait Action {
    async fn execute(&self) -> Result<()>;
}
