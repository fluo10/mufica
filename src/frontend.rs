mod matrix;

pub use matrix::{MatrixConfig, MatrixService};

use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FrontendConfig{
    Matrix(MatrixConfig),
}

use crate::{Result, ServiceExt};
pub enum FrontendService{
    Matrix(MatrixService),
}

impl ServiceExt for FrontendService{}
