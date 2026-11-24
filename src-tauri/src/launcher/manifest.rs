use crate::error::LauncherError;
use crate::instances::InstanceJson;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};
use std::fs;
use std::io::Write;
use std::path::Path;

const MANIFEST_URL: &str = "https://launchermeta.mojang.com/mc/game/version_manifest.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionManifest {
    pub latest: Latest,
    pub versions: Vec<VersionEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Latest {
    pub release: String,
    pub snapshot: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionEntry {
    pub id: String,
    pub r#type: String,
    pub url: String,
    pub time: String,
    pub release_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionJson {
    pub id: String,
    pub assets: String,
    pub asset_index: AssetIndex,
    pub downloads: Downloads,
    pub libraries: Vec<Library>,
    pub arguments: Option<GameArguments>,
    pub java_version: Option<JavaVersionRequirement>,
    pub main_class: String,
    pub minecraft_arguments: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetIndex {
    pub id: String,
    pub sha1: String,
    pub size: usize,
    pub url: String,
    pub total_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Downloads {
    pub client: DownloadInfo,
    pub server: Option<DownloadInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadInfo {
    pub sha1: String,
    pub size: usize,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Library {
    pub name: String,
    pub url: Option<String>,
    pub downloads: Option<LibraryDownloads>,
    pub rules: Option<Vec<LibraryRule>>,
    pub natives: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryDownloads {
    pub artifact: Option<LibraryArtifact>,
    pub classifiers: Option<serde_json::Map<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryArtifact {
    pub sha1: String,
    pub size: usize,
    pub url: String,
    pub path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryRule {
    pub action: String,
    pub os: Option<LibraryOs>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryOs {
    pub name: String,
    pub arch: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameArguments {
    pub game: Vec<serde_json::Value>,
    pub jvm: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JavaVersionRequirement {
    pub major_version: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DownloadProgress {
    pub current: usize,
    pub total: usize,
    pub file: String,
}

#[tauri::command]
pub async fn get_version_manifest() -> Result<VersionManifest, LauncherError> {
    let client = Client::new();
    let resp = client
        .get(MANIFEST_URL)
        .header("User-Agent", "MinecraftLauncher/0.1.0")
        .send()
        .await
        .map_err(|e| LauncherError::Http(e.to_string()))?;

    let manifest: VersionManifest = resp
        .json()
        .await
        .map_err(|e| LauncherError::Json(e.to_string()))?;

    Ok(manifest)
}

fn current_platform_os_name() -> &'static str {
    if cfg!(target_os = "windows") {
        "windows"
    } else if cfg!(target_os = "macos") {
        "osx"
    } else if cfg!(target_os = "linux") {
        "linux"
    } else {
        "windows"
    }
}

fn current_arch() -> &'static str {
    if cfg!(target_arch = "x86_64") {
        "x86_64"
    } else if cfg!(target_arch = "aarch64") {
        "aarch64"
    } else {
        "x86_64"
    }
}

pub fn library_applies(lib: &Library) -> bool {
    if let Some(rules) = &lib.rules {
        let mut allowed = true;
        for rule in rules {
            let os_match = rule.os.as_ref().map_or(true, |os| {
                os.name == current_platform_os_name()
                    && os.arch.as_ref().map_or(true, |a| a == current_arch())
            });
            match rule.action.as_str() {
                "allow" => allowed = os_match,
                "deny" => {
                    if os_match {
                        allowed = false;
                    }
                }
                _ => {}
            }
        }
        if !allowed {
            return false;
        }
    }
    true
}

pub fn library_native_classifier(lib: &Library) -> Option<String> {
    lib.natives.as_ref().and_then(|n| {
        n.get(current_platform_os_name())
            .and_then(|v| v.as_str())
            .map(|s| {
                s.replace("${arch}", current_arch())
                    .replace("${os}", current_platform_os_name())
            })
    })
}

fn sha1_hex(data: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

fn verify_sha1(path: &Path, expected: &str) -> Result<(), LauncherError> {
    let data = fs::read(path).map_err(|e| LauncherError::Io(e.to_string()))?;
    let actual = sha1_hex(&data);
    if actual != expected {
        return Err(LauncherError::HashMismatch {
            expected: expected.to_string(),
            actual,
        });
    }
    Ok(())
}

async fn download_file(
    client: &Client,
    url: &str,
    dest: &Path,
    expected_sha1: Option<&str>,
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

    if let Some(expected) = expected_sha1 {
        verify_sha1(dest, expected)?;
    }

    Ok(())
}

#[tauri::command]
pub async fn download_instance(
    state: tauri::State<'_, crate::AppState>,
    instance_id: String,
) -> Result<bool, LauncherError> {
    let config = &state.config;
    let json_path = config.instance_json_path(&instance_id);
    if !json_path.exists() {
        return Err(LauncherError::InstanceNotFound(instance_id.clone()));
    }
    let data = fs::read_to_string(&json_path).map_err(|e| LauncherError::Io(e.to_string()))?;
    let instance_json: InstanceJson = serde_json::from_str(&data).map_err(|e| LauncherError::Json(e.to_string()))?;

    let instance_dir = config.instance_dir(&instance_id);
    let bin_dir = instance_dir.join("bin");
    let libs_dir = instance_dir.join("libraries");
    let assets_dir = instance_dir.join("assets");
    let versions_dir = instance_dir.join("versions");

    fs::create_dir_all(&bin_dir).map_err(|e| LauncherError::Io(e.to_string()))?;
    fs::create_dir_all(&libs_dir).map_err(|e| LauncherError::Io(e.to_string()))?;
    fs::create_dir_all(&assets_dir).map_err(|e| LauncherError::Io(e.to_string()))?;
    fs::create_dir_all(&versions_dir).map_err(|e| LauncherError::Io(e.to_string()))?;

    let client = Client::new();

    let manifest = get_version_manifest().await?;
    let version_entry = manifest
        .versions
        .iter()
        .find(|v| v.id == instance_json.mc_version)
        .ok_or_else(|| LauncherError::DownloadFailed(format!("Version {} not found", instance_json.mc_version)))?;

    let version_json: VersionJson = client
        .get(&version_entry.url)
        .header("User-Agent", "MinecraftLauncher/0.1.0")
        .send()
        .await
        .map_err(|e| LauncherError::Http(e.to_string()))?
        .json()
        .await
        .map_err(|e| LauncherError::Json(e.to_string()))?;

    let version_json_path = versions_dir.join(format!("{}.json", instance_json.mc_version));
    let version_json_str =
        serde_json::to_string_pretty(&version_json).map_err(|e| LauncherError::Json(e.to_string()))?;
    fs::write(&version_json_path, version_json_str).map_err(|e| LauncherError::Io(e.to_string()))?;

    let client_info = &version_json.downloads.client;
    let client_jar_path = bin_dir.join(format!("{}.jar", instance_json.mc_version));
    download_file(
        &client,
        &client_info.url,
        &client_jar_path,
        Some(&client_info.sha1),
    )
    .await?;

    for lib in &version_json.libraries {
        if !library_applies(lib) {
            continue;
        }

        if let Some(downloads) = &lib.downloads {
            if let Some(artifact) = &downloads.artifact {
                let dest_path = libs_dir.join(artifact.path.as_deref().unwrap_or(&lib.name.replace(':', "/")));
                download_file(
                    &client,
                    &artifact.url,
                    &dest_path,
                    Some(&artifact.sha1),
                )
                .await?;
            }

            if let Some(classifiers) = &downloads.classifiers {
                if let Some(classifier) = library_native_classifier(lib) {
                    if let Some(native_artifact) = classifiers.get(&classifier) {
                        let dest_path = libs_dir.join(native_artifact.get("path").and_then(|v| v.as_str()).unwrap_or(&format!("{}-{}", lib.name, classifier)));
                        download_file(
                            &client,
                            &native_artifact.get("url").and_then(|v| v.as_str()).unwrap_or(""),
                            &dest_path,
                            Some(&native_artifact.get("sha1").and_then(|v| v.as_str()).unwrap_or("")),
                        )
                        .await?;
                    }
                }
            }
        }
    }

    let asset_index_path = assets_dir.join("indexes").join(format!("{}.json", version_json.asset_index.id));
    fs::create_dir_all(assets_dir.join("indexes")).map_err(|e| LauncherError::Io(e.to_string()))?;
    download_file(
        &client,
        &version_json.asset_index.url,
        &asset_index_path,
        Some(&version_json.asset_index.sha1),
    )
    .await?;

    Ok(true)
}

#[tauri::command]
pub async fn verify_instance_files(
    state: tauri::State<'_, crate::AppState>,
    instance_id: String,
) -> Result<bool, LauncherError> {
    let config = &state.config;
    let json_path = config.instance_json_path(&instance_id);
    if !json_path.exists() {
        return Err(LauncherError::InstanceNotFound(instance_id));
    }
    let data = fs::read_to_string(&json_path).map_err(|e| LauncherError::Io(e.to_string()))?;
    let _instance_json: InstanceJson = serde_json::from_str(&data).map_err(|e| LauncherError::Json(e.to_string()))?;

    let bin_dir = config.instance_dir(&instance_id).join("bin");
    let client_jar_path = bin_dir.join(format!("{}.jar", instance_id));

    if !client_jar_path.exists() {
        return Ok(false);
    }

    Ok(true)
}
