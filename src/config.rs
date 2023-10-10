use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    global: GlobalConfig,
    #[serde(with = "serde_yaml::with::singleton_map_recursive")]
    frontends: Vec<FrontendConfig>,
    #[serde(with = "serde_yaml::with::singleton_map")]
    backend: BackendConfig
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FrontendConfig{
    Matrix(MatrixConfig),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BackendConfig{
    TextGenerationWebui(TextGenerationWebuiConfig),
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
backend:
  text_generation_webui:
    model: test_model
frontends:
"#;
        let config: Config = serde_yaml::from_str(yaml).unwrap();
        assert_eq!( config, Config{
            global: GlobalConfig::default(),
            backend: BackendConfig::TextGenerationWebui(TextGenerationWebuiConfig{
                model: "test_model".to_string(),
            ..Default::default()
            }),
            frontends: Vec::new(),
        });
    }
}

