mod text_generation_webui;

pub use text_generation_webui::{TextGenerationWebui};

use crate::{Result};

use serde::{Serialize, Deserialize};


#[derive(Debug,)]
pub enum Backend{
    TextGenerationWebui(TextGenerationWebui),
}

