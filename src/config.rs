#[cfg(feature="text-generation-webui")]
mod text_generation_webui;

#[cfg(feature="matrix")]
pub use mufica_matrix::MatrixConfig;

#[cfg(feature="text-generation-webui")]
pub use text_generation_webui::TextGenerationWebuiConfig;

use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub global: GlobalConfig,
    #[serde(with = "serde_yaml::with::singleton_map_recursive")]
    pub frontends: Vec<FrontendConfig>,
    #[serde(with = "serde_yaml::with::singleton_map")]
    pub backend: BackendConfig
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GlobalConfig {
    pub config_dir: String,
    pub data_dir: String,
    pub interval: u64,
}

impl Default for GlobalConfig {
    fn default() -> Self {
        Self{
            config_dir: "/etc/tgwbot".to_string(),
            data_dir: "/var/lib/tgwbot".to_string(),
            interval: 10,
        }
    }
}



#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FrontendConfig{
    #[cfg(feature="matrix")]
    Matrix(MatrixConfig),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BackendConfig{
    #[cfg(feature="text-generation-webui")]
    TextGenerationWebui(TextGenerationWebuiConfig),
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    #[cfg(feature="text-generation-webui")]
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

