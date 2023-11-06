mod matrix;

pub use matrix::MatrixWorker;

use std::sync::Arc;
use tokio::sync::Mutex;
use crate::{FrontendConfig, Backend, MutexHistories, Result};


impl Worker {
    pub async fn new(backend: &Arc<Mutex<Backend>>, config: FrontendConfig) -> Result<Self> {
        Ok(match config {
            FrontendConfig::Matrix(x) => Self::Matrix(MatrixWorker::new(backend, x).await?)
        })
    }
    pub async fn sync(self) -> Result<()> {
        match self {
            Self::Matrix(x) => x.sync().await,
        }
    }
    pub async fn sync_once(self) -> Result<()> {
        match self {
            Self::Matrix(x) => x.sync_once().await,
        }
    }
}

pub enum Worker{
    Matrix(MatrixWorker),
}
