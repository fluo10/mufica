mod text_generation_webui;

pub use text_generation_webui::{TextGenerationWebuiBackend};

use crate::{Result};

use serde::{Serialize, Deserialize};


#[derive(Debug,)]
pub enum Backend{
    TextGenerationWebui(TextGenerationWebuiBackend),
}

