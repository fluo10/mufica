use std::sync::Arc;
use tokio::sync::Mutex;
use crate::{FrontendService, BackendService, LocalHistories, Result};

pub struct Service {
    pub frontend: FrontendService,
    pub backend: Arc<Mutex<BackendService>>,
    pub histories: LocalHistories,
}

impl Service {
    async fn sync(self) -> Result<()> {
        todo!()
    }
}

