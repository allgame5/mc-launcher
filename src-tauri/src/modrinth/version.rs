use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstalledMod {
    pub project_id: String,
    pub version_id: String,
    pub version_number: String,
    pub project_title: String,
    pub file_name: String,
    pub sha512: String,
    pub installed_path: PathBuf,
    pub installed_at: String,
}
