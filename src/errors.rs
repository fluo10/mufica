use std::convert::From;
use std::io;
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Join error")]
    Join(tokio::task::JoinError),
    #[error("IO error")]
    Io(io::Error),
    #[cfg(feature="matrix")]
    #[error("Matrix Error")]
    Matrix(mufica_matrix::Error),
    #[error("Json parse error")]
    ParseJson(serde_json::Error),
    #[error("Parse yaml error")]
    ParseYaml(serde_yaml::Error),
    #[error("Response error")]
    Response(reqwest::Error),
    #[error("Parse Url error")]
    ParseUrl(url::ParseError),
    #[cfg(feature="cli")]
    #[error("Command parse error")]
    Cli(clap::Error),
    #[cfg(feature="text-generation-webui")]
    #[error("Text generation webui api error")]
    TextGenerationWebuiApi(text_generation_webui_api::Error),
}

impl From<tokio::task::JoinError> for Error {
    fn from(e: tokio::task::JoinError) -> Self {
        Self::Join(e)
    }
}
impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}
#[cfg(feature="matrix")]
impl From<mufica_matrix::Error> for Error {
    fn from(e: mufica_matrix::Error) -> Self {
        Self::Matrix(e)
    }
}
impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::ParseJson(e)
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(e: serde_yaml::Error) -> Self {
        Self::ParseYaml(e)
    }
}
impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::Response(e)
    }
}
impl From<url::ParseError> for Error {
    fn from(e: url::ParseError) -> Self {
        Self::ParseUrl(e)
    }
}
#[cfg(feature="cli")]
impl From<clap::Error> for Error {
    fn from(e: clap::Error) -> Self {
        Self::Cli(e)
    }
}

#[cfg(feature="text-generation-webui")]
impl From<text_generation_webui_api::Error> for Error {
    fn from(e: text_generation_webui_api::Error) -> Self {
        Self::TextGenerationWebuiApi(e)
    }
}

