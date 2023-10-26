use crate::Result;
use serde::{Deserialize, Serialize};
use text_generation_webui_api::{ChatApiRequest, ChatApiResponse, ModelApiRequest, ModelApiResponse, ModelApiResponseResult, Character, History};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TextGenerationWebuiConfig {
    pub host: String,
    pub character: Option<String>,
    pub model: String,
    pub update_history: bool,
    pub character_file: String,
    pub history_file: String,
}

impl Default for TextGenerationWebuiConfig {
    fn default() -> Self {
        Self{
            host: "http://localhost:5000".to_string(),
            character: None,
            model: "".to_string(),
            update_history: false,
            character_file: "/etc/tgwbot/characters".to_string(),
            history_file: "/var/lib/tgwbot/logs".to_string(),
        }
    }
}
impl TextGenerationWebuiConfig {

    pub fn get_history(&self) -> Result<text_generation_webui_api::History> {
        todo!();
    }
    pub fn get_character(&self) -> Result<Character> {
        todo!();
    }
}
