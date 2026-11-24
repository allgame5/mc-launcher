use thiserror::Error;

#[derive(Error, Debug, serde::Serialize)]
#[serde(tag = "type", content = "message")]
pub enum LauncherError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Zip error: {0}")]
    Zip(#[from] zip::result::ZipError),

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

impl From<LauncherError> for String {
    fn from(err: LauncherError) -> Self {
        err.to_string()
    }
}
