use serde::{Serialize, Deserialize};

pub struct Config {
    global: GlobalConfig,
    text_generation_webui: TextGenerationWebuiConfig,
    matrix: Option<MatrixConfig>,
}


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GlobalConfig {
    #[serde(default)]
    config_dir: String,
    #[serde(default)]
    data_dir: String,
}

impl Default for GlobalConfig {
    fn default() -> Self {
        Self{
            config_dir: "/etc/tgwbot".to_string(),
            data_dir: "/var/lib/tgwbot".to_string(),
        }
    }
}


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TextGenerationWebuiConfig {
    host: String,
    character: Option<String>,
    model: String,
    update_history: bool,
    #[serde(default)]
    character_dir: String,
    #[serde(default)]
    history_dir: String,
}

impl Default for TextGenerationWebuiConfig {
    fn default() -> Self {
        Self{
            host: "".to_string(),
            character: None,
            model: "".to_string(),
            update_history: false,
            character_dir: "/etc/tgwbot/characters".to_string(),
            history_dir: "/var/lib/tgwbot/logs".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MatrixConfig {
    host: String,
    user: Option<String>,
    password: Option<String>,
    token: Option<String>,
}
