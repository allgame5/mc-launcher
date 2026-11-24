use crate::error::LauncherError;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FabricLoaderVersion {
    pub loader: String,
    pub stable: bool,
    pub maven_url: String,
}

const FABRIC_META_URL: &str = "https://meta.fabricmc.net/v2/versions/loader";

#[tauri::command]
pub async fn install_fabric(
    state: tauri::State<'_, crate::AppState>,
    instance_id: String,
    mc_version: String,
) -> Result<bool, LauncherError> {
    let config = &state.config;
    let instance_dir = config.instance_dir(&instance_id);
    let libs_dir = instance_dir.join("libraries");
    let bin_dir = instance_dir.join("bin");

    fs::create_dir_all(&libs_dir).map_err(|e| LauncherError::Io(e.to_string()))?;
    fs::create_dir_all(&bin_dir).map_err(|e| LauncherError::Io(e.to_string()))?;

    let client = Client::new();

    let url = format!("{}/{}", FABRIC_META_URL, mc_version);
    let versions: Vec<FabricLoaderVersion> = client
        .get(&url)
        .header("User-Agent", "MinecraftLauncher/0.1.0")
        .send()
        .await
        .map_err(|e| LauncherError::Http(e.to_string()))?
        .json()
        .await
        .map_err(|e| LauncherError::Http(e.to_string()))?;

    let stable = versions
        .iter()
        .find(|v| v.stable)
        .ok_or_else(|| LauncherError::DownloadFailed("No stable fabric loader found".to_string()))?;

    let installer_path = bin_dir.join("fabric-installer.jar");
    download_file(&client, &stable.maven_url, &installer_path, None).await?;

    Ok(true)
}

async fn download_file(
    client: &Client,
    url: &str,
    dest: &Path,
    _expected_sha1: Option<&str>,
) -> Result<(), LauncherError> {
    let mut resp = client
        .get(url)
        .header("User-Agent", "MinecraftLauncher/0.1.0")
        .send()
        .await
        .map_err(|e| LauncherError::Http(e.to_string()))?;

    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent).map_err(|e| LauncherError::Io(e.to_string()))?;
    }

    let mut file = fs::File::create(dest).map_err(|e| LauncherError::Io(e.to_string()))?;
    while let Some(chunk) = resp.chunk().await.map_err(|e| LauncherError::Http(e.to_string()))? {
        file.write_all(&chunk).map_err(|e| LauncherError::Io(e.to_string()))?;
    }

    Ok(())
}
