#[cfg(feature="text-generation-webui")]
mod text_generation_webui;

#[cfg(feature="text-generation-webui")]
pub use text_generation_webui::{TextGenerationWebuiBackend};

use crate::{Result};

use serde::{Serialize, Deserialize};


#[derive(Debug,)]
pub enum Backend{
    #[cfg(feature="text-generation-webui")]
    TextGenerationWebui(TextGenerationWebuiBackend),
}

