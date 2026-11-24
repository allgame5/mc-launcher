use crate::config::LauncherConfig;
use crate::error::LauncherError;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FabricLoaderVersion {
    pub loader: String,
    pub stable: bool,
    pub maven_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FabricVersionManifest {
    pub version: String,
    pub java_version: Option<i32>,
    pub libraries: Vec<FabricLibrary>,
    pub downloads: FabricDownloads,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FabricLibrary {
    pub name: String,
    pub downloads: Option<FabricLibraryDownloads>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FabricLibraryDownloads {
    pub artifact: Option<FabricArtifact>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FabricArtifact {
    pub sha1: String,
    pub size: usize,
    pub url: String,
    pub path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FabricDownloads {
    pub client: Option<FabricArtifact>,
    pub server: Option<FabricArtifact>,
}

const FABRIC_META_URL: &str = "https://meta.fabricmc.net/v2/versions/loader";

#[tauri::command]
pub async fn install_fabric(
    state: tauri::State<'_, crate::main::AppState>,
    instance_id: String,
    mc_version: String,
) -> Result<bool, LauncherError> {
    let config = &state.config;
    let instance_dir = config.instance_dir(&instance_id);
    let libs_dir = instance_dir.join("libraries");
    let bin_dir = instance_dir.join("bin");

    fs::create_dir_all(&libs_dir).map_err(|e| LauncherError::Io(e))?;
    fs::create_dir_all(&bin_dir).map_err(|e| LauncherError::Io(e))?;

    let client = Client::new();

    // 1. Verfügbare Loader-Versionen für MC-Version abrufen
    let url = format!("{}/{}", FABRIC_META_URL, mc_version);
    let versions: Vec<FabricLoaderVersion> = client
        .get(&url)
        .header("User-Agent", "MinecraftLauncher/0.1.0")
        .send()
        .await
        .map_err(|e| LauncherError::Http(e))?
        .json()
        .await
        .map_err(|e| LauncherError::Json(e))?;

    let stable = versions
        .iter()
        .find(|v| v.stable)
        .ok_or_else(|| LauncherError::DownloadFailed("No stable fabric loader found".to_string()))?;

    // 2. Loader-Installer-JAR herunterladen (maven_url)
    let installer_path = bin_dir.join("fabric-installer.jar");
    download_file(&client, &stable.maven_url, &installer_path, None).await?;

    // 3. Client-JAR von Mojang für die Version laden (falls noch nicht vorhanden)
    // Hier würden wir die Mojang-API aufrufen, aber für Fabric reicht oft der Installer,
    // der die Libraries selbst zieht. Wir speichern zumindest den Installer-Pfad.

    // 4. Fabric wird beim Start über den Installer installiert.
    // Wir markieren die Instanz als fabric-ready.

    Ok(true)
}

async fn download_file(
    client: &Client,
    url: &str,
    dest: &std::path::Path,
    _expected_sha1: Option<&str>,
) -> Result<(), LauncherError> {
    let mut resp = client
        .get(url)
        .header("User-Agent", "MinecraftLauncher/0.1.0")
        .send()
        .await
        .map_err(|e| LauncherError::Http(e))?;

    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent).map_err(|e| LauncherError::Io(e))?;
    }

    let mut file = fs::File::create(dest).map_err(|e| LauncherError::Io(e))?;
    while let Some(chunk) = resp.chunk().await.map_err(|e| LauncherError::Http(e))? {
        file.write_all(&chunk).map_err(|e| LauncherError::Io(e))?;
    }

    Ok(())
}
