use serde::{Serialize, Deserialize};

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    global: GlobalConfig,
    text_generation_webui: TextGenerationWebuiConfig,
    matrix: Option<MatrixConfig>,
}



#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GlobalConfig {
    config_dir: String,
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
#[serde(default)]
pub struct TextGenerationWebuiConfig {
    host: String,
    character: Option<String>,
    model: String,
    update_history: bool,
    character_dir: String,
    history_dir: String,
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

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MatrixConfig {
    host: String,
    user: Option<String>,
    password: Option<String>,
    token: Option<String>,
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn parse_minimum_config(){
        let yaml = r#"global:
text_generation_webui:
  model: test_model
"#;
        let config: Config = serde_yaml::from_str(yaml).unwrap();
        assert_eq!( config, Config{
            global: GlobalConfig::default(),
            text_generation_webui: TextGenerationWebuiConfig{
                model: "test_model".to_string(),
            ..Default::default()
            },
            matrix: None,
        });
    }
}

