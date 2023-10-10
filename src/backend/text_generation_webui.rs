use serde::{Serialize, Deserialize};
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TextGenerationWebuiConfig {
    pub host: String,
    pub character: Option<String>,
    pub model: String,
    pub update_history: bool,
    pub character_dir: String,
    pub history_dir: String,
}

impl Default for TextGenerationWebuiConfig {
    fn default() -> Self {
        Self{
            host: "http://localhost:5000".to_string(),
            character: None,
            model: "".to_string(),
            update_history: false,
            character_dir: "/etc/tgwbot/characters".to_string(),
            history_dir: "/var/lib/tgwbot/logs".to_string(),
        }
    }
}

pub struct TextGenerationWebuiService {
}
