use crate::{MatrixClient, Result};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::Path
};


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MatrixConfig {
    pub host: String,
    pub user: Option<String>,
    pub password: Option<String>,
    pub token: Option<String>,
    pub data_dir: String,
}

impl MatrixConfig {
    pub fn read_file(path: &impl AsRef<Path>) -> Result<Self> {
         let content = fs::read_to_string(path).unwrap();
         let config: MatrixConfig = serde_yaml::from_str(&content)?;
         Ok(config)
    }
    pub fn to_clients(&self) -> Result<Vec<MatrixClient>> {
        todo!()
    }
}

