#[cfg(feature="matrix")]
pub use mufica_matrix::MatrixClient;

use std::sync::Arc;
use tokio::sync::Mutex;
use crate::{FrontendConfig, Backend, MutexHistories, Result};


impl Worker {
    pub async fn sync(self) -> Result<()> {
        match self {
            #[cfg(feature="matrix")]
            Self::Matrix(x) => x.sync(|x| {todo!()}).await?,
        }
        Ok(())
    }
    pub async fn sync_once(self) -> Result<()> {
        match self {
            #[cfg(feature="matrix")]
            Self::Matrix(x) => x.sync_once().await?,
        }
        Ok(())
    }
}

pub enum Worker{
    #[cfg(feature="matrix")]
    Matrix(MatrixClient),
}
