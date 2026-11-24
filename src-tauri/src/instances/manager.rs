use crate::config::LauncherConfig;
use crate::error::LauncherError;
use crate::instances::model::{Instance, InstanceJson, ModLoader};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::State;
use uuid::Uuid;

use crate::launcher::java::ensure_java_for_version;

pub struct AppState {
    pub config: LauncherConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateInstanceRequest {
    pub name: String,
    pub mc_version: String,
    pub mod_loader: String,
    pub loader_version: Option<String>,
    pub ram_mb: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EditInstanceRequest {
    pub id: String,
    pub name: Option<String>,
    pub ram_mb: Option<u32>,
    pub icon: Option<String>,
}

#[tauri::command]
pub fn list_instances(
    state: State<'_, AppState>,
) -> Result<Vec<InstanceJson>, LauncherError> {
    let instances_dir = &state.config.paths().instances_dir;
    fs::create_dir_all(instances_dir).map_err(|e| LauncherError::Io(e))?;

    let mut instances = Vec::new();
    if let Ok(entries) = fs::read_dir(instances_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let json_path = path.join("instance.json");
                if json_path.exists() {
                    let data = fs::read_to_string(&json_path)
                        .map_err(|e| LauncherError::Io(e))?;
                    let instance_json: InstanceJson =
                        serde_json::from_str(&data).map_err(|e| LauncherError::Json(e))?;
                    instances.push(instance_json);
                }
            }
        }
    }

    instances.sort_by(|a, b| {
        b.created_at
            .cmp(&a.created_at)
    });

    Ok(instances)
}

#[tauri::command]
pub fn get_instance(
    state: State<'_, AppState>,
    instance_id: String,
) -> Result<InstanceJson, LauncherError> {
    let json_path = state.config.instance_json_path(&instance_id);
    if !json_path.exists() {
        return Err(LauncherError::InstanceNotFound(instance_id));
    }
    let data = fs::read_to_string(&json_path).map_err(|e| LauncherError::Io(e))?;
    let instance_json: InstanceJson = serde_json::from_str(&data).map_err(|e| LauncherError::Json(e))?;
    Ok(instance_json)
}

#[tauri::command]
pub fn create_instance(
    state: State<'_, AppState>,
    request: CreateInstanceRequest,
) -> Result<InstanceJson, LauncherError> {
    let id = Uuid::new_v4();
    let instance_id = id.to_string();
    let instance_dir = state.config.instance_dir(&instance_id);

    // Verzeichnisstruktur anlegen
    fs::create_dir_all(&instance_dir).map_err(|e| LauncherError::Io(e))?;
    fs::create_dir_all(instance_dir.join("mods")).map_err(|e| LauncherError::Io(e))?;
    fs::create_dir_all(instance_dir.join("config")).map_err(|e| LauncherError::Io(e))?;
    fs::create_dir_all(instance_dir.join("logs")).map_err(|e| LauncherError::Io(e))?;
    if request.mod_loader == "fabric" || request.mod_loader == "neoforge" {
        fs::create_dir_all(instance_dir.join("mods")).map_err(|e| LauncherError::Io(e))?;
    }

    let ram_mb = request.ram_mb.unwrap_or(state.config.default_ram_mb);

    let instance = Instance {
        id,
        name: request.name,
        mc_version: request.mc_version.clone(),
        mod_loader: ModLoader::from_str(&request.mod_loader)?,
        loader_version: request.loader_version.clone(),
        ram_mb,
        icon: None,
        last_played: None,
        created_at: chrono::Utc::now(),
    };

    let instance_json = InstanceJson::from(&instance);
    let json_data = serde_json::to_string_pretty(&instance_json)
        .map_err(|e| LauncherError::Json(e))?;
    fs::write(instance_dir.join("instance.json"), &json_data)
        .map_err(|e| LauncherError::Io(e))?;

    // Java für die Version sicherstellen (asynchron im Hintergrund nicht blockierend hier)
    let _ = ensure_java_for_version(&state.config, &request.mc_version);

    Ok(instance_json)
}

#[tauri::command]
pub fn edit_instance(
    state: State<'_, AppState>,
    request: EditInstanceRequest,
) -> Result<InstanceJson, LauncherError> {
    let mut instance_json = get_instance(state.clone(), request.id.clone())?;

    if let Some(name) = request.name {
        instance_json.name = name;
    }
    if let Some(ram_mb) = request.ram_mb {
        instance_json.ram_mb = ram_mb;
    }
    if let Some(icon) = request.icon {
        instance_json.icon = Some(icon);
    }

    let json_path = state.config.instance_json_path(&instance_json.id);
    let json_data = serde_json::to_string_pretty(&instance_json)
        .map_err(|e| LauncherError::Json(e))?;
    fs::write(&json_path, json_data).map_err(|e| LauncherError::Io(e))?;

    Ok(instance_json)
}

#[tauri::command]
pub fn delete_instance(
    state: State<'_, AppState>,
    instance_id: String,
) -> Result<bool, LauncherError> {
    let instance_dir = state.config.instance_dir(&instance_id);
    if instance_dir.exists() {
        fs::remove_dir_all(&instance_dir).map_err(|e| LauncherError::Io(e))?;
    }
    Ok(true)
}
