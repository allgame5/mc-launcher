use crate::error::LauncherError;
use crate::modrinth::client::ModrinthClient;
use crate::modrinth::version::InstalledMod;
use serde::Deserialize;
use sha2::{Digest, Sha512};
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use tauri::State;

#[derive(Debug, Deserialize)]
pub struct InstallModRequest {
    pub instance_id: String,
    pub project_id: String,
    pub version_id: String,
    pub project_type: String,
}

#[derive(Debug, Serialize)]
pub struct InstallResult {
    pub installed: Vec<InstalledMod>,
    pub skipped: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct ListInstalledModsRequest {
    pub instance_id: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateModsRequest {
    pub instance_id: String,
}

fn mod_folder_for_type(project_type: &str) -> &'static str {
    match project_type {
        "mod" => "mods",
        "shaderpack" => "shaderpacks",
        "resourcepack" => "resourcepacks",
        _ => "mods",
    }
}

fn installed_mods_path(config: &crate::config::LauncherConfig, instance_id: &str) -> PathBuf {
    config.instance_dir(instance_id).join("installed_mods.json")
}

fn load_installed_mods(config: &crate::config::LauncherConfig, instance_id: &str) -> Vec<InstalledMod> {
    let path = installed_mods_path(config, instance_id);
    if !path.exists() {
        return Vec::new();
    }
    let data = match fs::read_to_string(&path) {
        Ok(d) => d,
        Err(_) => return Vec::new(),
    };
    serde_json::from_str(&data).unwrap_or_default()
}

fn save_installed_mods(config: &crate::config::LauncherConfig, instance_id: &str, mods: &[InstalledMod]) {
    let path = installed_mods_path(config, instance_id);
    let _ = fs::write(
        &path,
        serde_json::to_string_pretty(mods).unwrap_or_default(),
    );
}

#[tauri::command]
pub async fn install_modrinth_project(
    _app_handle: tauri::AppHandle,
    state: State<'_, crate::AppState>,
    request: InstallModRequest,
) -> Result<InstallResult, LauncherError> {
    let config = &state.config;
    let instance_dir = config.instance_dir(&request.instance_id);
    let folder = mod_folder_for_type(&request.project_type);
    let target_dir = instance_dir.join(folder);
    fs::create_dir_all(&target_dir).map_err(|e| LauncherError::Io(e))?;

    let client = ModrinthClient::new();
    let mut installed = Vec::new();
    let mut skipped = Vec::new();
    let mut visited = HashSet::new();

    install_version_recursive(
        &client,
        config,
        &request.instance_id,
        &request.project_id,
        &request.version_id,
        &target_dir,
        &mut installed,
        &mut skipped,
        &mut visited,
    )
    .await?;

    save_installed_mods(config, &request.instance_id, &installed);

    Ok(InstallResult { installed, skipped })
}

async fn install_version_recursive(
    client: &ModrinthClient,
    config: &crate::config::LauncherConfig,
    instance_id: &str,
    project_id: &str,
    version_id: &str,
    target_dir: &std::path::Path,
    installed: &mut Vec<InstalledMod>,
    skipped: &mut Vec<String>,
    visited: &mut HashSet<String>,
) -> Result<(), LauncherError> {
    if visited.contains(project_id) {
        return Ok(());
    }
    visited.insert(project_id.to_string());

    let versions = client.get_project_versions(project_id, None, None).await?;
    let version = versions
        .iter()
        .find(|v| v.id == version_id)
        .ok_or_else(|| LauncherError::Modrinth(format!("Version {} not found", version_id)))?;

    let primary_file = version
        .files
        .iter()
        .find(|f| f.primary)
        .or_else(|| version.files.first())
        .ok_or_else(|| LauncherError::Modrinth("No files in version".to_string()))?;

    let dest_path = target_dir.join(&primary_file.filename);

    if dest_path.exists() {
        let data = fs::read(&dest_path).map_err(|e| LauncherError::Io(e))?;
        let mut hasher = Sha512::new();
        hasher.update(&data);
        let actual = format!("{:x}", hasher.finalize());
        if actual == primary_file.hashes.sha512 {
            skipped.push(format!("{} (already installed)", primary_file.filename));
            installed.push(InstalledMod {
                project_id: project_id.to_string(),
                version_id: version.id.clone(),
                version_number: version.version_number.clone(),
                project_title: project_id.to_string(),
                file_name: primary_file.filename.clone(),
                sha512: primary_file.hashes.sha512.clone(),
                installed_path: dest_path,
                installed_at: chrono::Utc::now().to_rfc3339(),
            });
            return Ok(());
        }
    }

    client.download_file(&primary_file.url, &dest_path, &primary_file.hashes.sha512).await?;

    installed.push(InstalledMod {
        project_id: project_id.to_string(),
        version_id: version.id.clone(),
        version_number: version.version_number.clone(),
        project_title: project_id.to_string(),
        file_name: primary_file.filename.clone(),
        sha512: primary_file.hashes.sha512.clone(),
        installed_path: dest_path,
        installed_at: chrono::Utc::now().to_rfc3339(),
    });

    for dep in &version.dependencies {
        if dep.dependency_type == "required" {
            let _ = install_version_recursive(
                client,
                config,
                instance_id,
                &dep.project_id,
                &dep.version_id,
                target_dir,
                installed,
                skipped,
                visited,
            )
            .await;
        }
    }

    Ok(())
}

#[tauri::command]
pub async fn list_installed_mods(
    state: State<'_, crate::AppState>,
    request: ListInstalledModsRequest,
) -> Result<Vec<InstalledMod>, LauncherError> {
    let config = &state.config;
    Ok(load_installed_mods(config, &request.instance_id))
}

#[tauri::command]
pub async fn update_installed_mods(
    _app_handle: tauri::AppHandle,
    state: State<'_, crate::AppState>,
    request: UpdateModsRequest,
) -> Result<InstallResult, LauncherError> {
    let config = &state.config;
    let installed = load_installed_mods(config, &request.instance_id);
    let client = ModrinthClient::new();

    let mut updated = Vec::new();
    let mut skipped = Vec::new();
    let mut visited = HashSet::new();

    for mod_entry in &installed {
        if visited.contains(&mod_entry.project_id) {
            continue;
        }
        visited.insert(mod_entry.project_id.clone());

        let versions = client
            .get_project_versions(&mod_entry.project_id, None, None)
            .await?;

        if let Some(latest) = versions.first() {
            if latest.id != mod_entry.version_id {
                let instance_dir = config.instance_dir(&request.instance_id);
                let folder = mod_folder_for_type("mod");
                let target_dir = instance_dir.join(folder);
                fs::create_dir_all(&target_dir).map_err(|e| LauncherError::Io(e))?;

                let old_path = &mod_entry.installed_path;
                if old_path.exists() {
                    let _ = fs::remove_file(old_path);
                }

                install_version_recursive(
                    &client,
                    config,
                    &request.instance_id,
                    &mod_entry.project_id,
                    &latest.id,
                    &target_dir,
                    &mut updated,
                    &mut skipped,
                    &mut visited,
                )
                .await?;
            } else {
                skipped.push(format!("{} (up to date)", mod_entry.project_id));
            }
        }
    }

    let all_installed = load_installed_mods(config, &request.instance_id);
    save_installed_mods(config, &request.instance_id, &all_installed);

    Ok(InstallResult {
        installed: updated,
        skipped,
    })
}
