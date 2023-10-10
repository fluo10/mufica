use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MatrixConfig {
    pub host: String,
    pub user: Option<String>,
    pub password: Option<String>,
    pub token: Option<String>,
}

pub struct MatrixService{
}
