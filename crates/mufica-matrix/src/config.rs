use crate::{MatrixClient, Result, MatrixSubscriber};
use imbl::Vector;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::Path
};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MatrixConfig {
    pub host: String,
    pub username: String,
    pub password: String,
    pub data_dir: String,
}

impl MatrixConfig {
    pub fn read_file(path: &impl AsRef<Path>) -> Result<Self> {
         let content = fs::read_to_string(path).unwrap();
         let config: MatrixConfig = serde_yaml::from_str(&content)?;
         Ok(config)
    }
    pub fn to_client(&self) -> Result<MatrixClient> {
        todo!()
    }
    pub fn to_subscribers(&self) -> Result<Vec<MatrixSubscriber>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_config () {
        let config_text = r#"host: "example.com",
username: "test",
password: "1234",
pub data_dir: "/var/lib/mufica/matrix"
"#;
        assert_eq!(MatrixConfig{
            host: "example.com".to_string(),
            username: "test".to_string(),
            password: "1234".to_string(),
            data_dir: "/var/lib/mufica/matrix".to_string(),
        }, serde_yaml.to_string(&config_text));
    }
}

