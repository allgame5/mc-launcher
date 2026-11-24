use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug, Serialize)]
#[serde(tag = "type", content = "message")]
pub enum LauncherError {
    #[error("IO error: {0}")]
    Io(String),

    #[error("HTTP error: {0}")]
    Http(String),

    #[error("JSON error: {0}")]
    Json(String),

    #[error("Zip error: {0}")]
    Zip(String),

    #[error("SHA1 error: {0}")]
    Sha1(String),

    #[error("SHA256/SHA512 error: {0}")]
    Sha2(String),

    #[error("Instance not found: {0}")]
    InstanceNotFound(String),

    #[error("Instance already exists: {0}")]
    InstanceAlreadyExists(String),

    #[error("Java not found: {0}")]
    JavaNotFound(String),

    #[error("Download failed: {0}")]
    DownloadFailed(String),

    #[error("Hash verification failed: expected {expected}, got {actual}")]
    HashMismatch { expected: String, actual: String },

    #[error("Auth error: {0}")]
    Auth(String),

    #[error("Modrinth API error: {0}")]
    Modrinth(String),

    #[error("Process error: {0}")]
    Process(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl From<String> for LauncherError {
    fn from(s: String) -> Self {
        LauncherError::Unknown(s)
    }
}

impl From<LauncherError> for String {
    fn from(err: LauncherError) -> Self {
        err.to_string()
    }
}
