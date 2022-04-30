use std::pin::Pin;

use futures::{AsyncRead, AsyncReadExt};
use ipi_core::anyhow::Result;

use crate::path::Path;

#[async_trait]
pub trait Storage {
    fn get_raw(&self, path: &Path) -> Result<Pin<Box<dyn AsyncRead + Send>>>;

    async fn get_raw_to_end(&self, path: &Path) -> Result<Vec<u8>> {
        let mut buf = Vec::with_capacity(path.len as usize);
        self.get_raw(path)?.read_to_end(&mut buf).await?;
        Ok(buf)
    }
}
