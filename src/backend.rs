mod text_generation_webui;

pub use text_generation_webui::{TextGenerationWebuiConfig, TextGenerationWebuiService};

use crate::{Result, ServiceExt};

use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BackendConfig{
    TextGenerationWebui(TextGenerationWebuiConfig),
}

pub enum BackendService{
    TextGenerationWebui(TextGenerationWebuiService),
}

impl ServiceExt for BackendService{}
