#[cfg(feature="matrix")]
pub use mufica_matrix::MatrixSubscriber;

use std::sync::Arc;
use std::convert::From;
use tokio::sync::Mutex;
use crate::{FrontendConfig, Backend, MutexHistories};
use crate::errors::{Result, Error};



impl Subscriber {
    pub async fn sync(self) -> Result<()> {
        match self {
            #[cfg(feature="matrix")]
            Self::Matrix(x) => todo!(),
        }
        Ok(())
    }
    pub async fn sync_once(self) -> Result<()> {
        match self {
            #[cfg(feature="matrix")]
            Self::Matrix(x) => todo!(),
        }
        Ok(())
    }
}

pub enum Subscriber{
    #[cfg(feature="matrix")]
    Matrix(MatrixSubscriber),
}

impl From<MatrixSubscriber> for Subscriber {
    fn from(m: MatrixSubscriber) -> Self {
        Subscriber::Matrix(m)
    }
}
