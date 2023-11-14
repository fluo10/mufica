use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MatrixConfig {
    pub host: String,
    pub user: Option<String>,
    pub password: Option<String>,
    pub token: Option<String>,
    pub data_dir: String,
}
