use core::pin::Pin;

use async_trait::async_trait;
use ipi::anyhow::Result;
use tokio::io::{AsyncRead, AsyncWrite, AsyncWriteExt};

#[async_trait]
pub trait Resource {
    async fn release(&mut self) -> Result<()>;
}

#[async_trait]
impl Resource for Pin<Box<dyn AsyncRead + Send + Sync>> {
    async fn release(&mut self) -> Result<()> {
        Ok(())
    }
}

#[async_trait]
impl Resource for Pin<Box<dyn AsyncWrite + Send + Sync>> {
    async fn release(&mut self) -> Result<()> {
        self.shutdown().await.map_err(Into::into)
    }
}
