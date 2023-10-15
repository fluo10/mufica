mod matrix;

pub use matrix::{MatrixConfig, MatrixService, MatrixHistory};
use crate::History;

use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FrontendConfig{
    Matrix(MatrixConfig),
}

pub enum FrontendService{
    Matrix(MatrixService),
}

