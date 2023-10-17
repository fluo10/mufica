mod text_generation_webui;

pub use text_generation_webui::{TextGenerationWebuiConfig, TextGenerationWebui};

use crate::{Result};

use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BackendConfig{
    TextGenerationWebui(TextGenerationWebuiConfig),
}

#[derive(Debug,)]
pub enum Backend{
    TextGenerationWebui(TextGenerationWebui),
}

