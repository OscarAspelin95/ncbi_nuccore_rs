use reqwest;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Request error: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("Download error: {0}")]
    StatusCodeError(String),

    #[error("Invalid request response: {0}")]
    InvalidResponseError(String),
}
