use ipi_core::anyhow::Result;

use crate::path::Path;

#[async_trait]
pub trait Storage {
    async fn get_raw_cloned(path: &Path) -> Result<Vec<u8>>;
}
