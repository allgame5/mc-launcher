use crate::error::LauncherError;\nuse serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LauncherConfig {
    pub default_ram_mb: u32,
    pub java_path: Option<PathBuf>,
    pub auto_update: bool,
    pub client_id: Option<String>,
    pub app_data_dir: PathBuf,
}

impl Default for LauncherConfig {
    fn default() -> Self {
        Self {
            default_ram_mb: 4096,
            java_path: None,
            auto_update: true,
            client_id: None,
            app_data_dir: PathBuf::new(),
        }
    }
}

pub struct AppPaths {
    pub instances_dir: PathBuf,
    pub java_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub logs_dir: PathBuf,
}

impl LauncherConfig {
    pub fn load_raw() -> Result<Self, crate::error::LauncherError> {
        let app_data_dir = default_app_data_dir();
        fs::create_dir_all(&app_data_dir).map_err(|e| LauncherError::Config(e.to_string()))?;

        let config_path = app_data_dir.join("launcher.json");
        let config = if config_path.exists() {
            let data = fs::read_to_string(&config_path)
                .map_err(|e| LauncherError::Config(e.to_string()))?;
            serde_json::from_str(&data).unwrap_or_default()
        } else {
            let mut cfg = LauncherConfig::default();
            cfg.app_data_dir = app_data_dir.clone();
            if let Ok(client_id) = std::env::var("MINECRAFT_LAUNCHER_CLIENT_ID") {
                if !client_id.is_empty() {
                    cfg.client_id = Some(client_id);
                }
            }
            cfg
        };

        Ok(config)
    }

    pub fn load(app_handle: &tauri::AppHandle) -> Result<Self, crate::error::LauncherError> {
        let app_data_dir = app_handle
            .path()
            .app_data_dir()
            .map_err(|e| LauncherError::Config(e.to_string()))?;

        fs::create_dir_all(&app_data_dir).map_err(|e| LauncherError::Config(e.to_string()))?;

        let config_path = app_data_dir.join("launcher.json");
        let config = if config_path.exists() {
            let data = fs::read_to_string(&config_path)
                .map_err(|e| LauncherError::Config(e.to_string()))?;
            serde_json::from_str(&data).unwrap_or_default()
        } else {
            let mut cfg = LauncherConfig::default();
            cfg.app_data_dir = app_data_dir.clone();
            // Client-ID aus Umgebungsvariable lesen
            if let Ok(client_id) = std::env::var("MINECRAFT_LAUNCHER_CLIENT_ID") {
                if !client_id.is_empty() {
                    cfg.client_id = Some(client_id);
                }
            }
            cfg
        };

        Ok(config)
    }

    pub fn save(&self, app_handle: &tauri::AppHandle) -> Result<(), crate::error::LauncherError> {
        let config_path = self
            .app_data_dir
            .join("launcher.json");
        let data = serde_json::to_string_pretty(self)
            .map_err(|e| LauncherError::Config(e.to_string()))?;
        fs::write(&config_path, data).map_err(|e| LauncherError::Config(e.to_string()))?;
        Ok(())
    }

    pub fn paths(&self) -> AppPaths {
        let instances_dir = self.app_data_dir.join("instances");
        let java_dir = self.app_data_dir.join("java");
        let cache_dir = self.app_data_dir.join("cache");
        let logs_dir = self.app_data_dir.join("logs");
        AppPaths {
            instances_dir,
            java_dir,
            cache_dir,
            logs_dir,
        }
    }

    pub fn instance_dir(&self, instance_id: &str) -> PathBuf {
        self.paths().instances_dir.join(instance_id)
    }

    pub fn instance_json_path(&self, instance_id: &str) -> PathBuf {
        self.instance_dir(instance_id).join("instance.json")
    }
}

#[tauri::command]
pub fn get_config(state: tauri::State<'_, crate::AppState>) -> Result<crate::config::LauncherConfig, LauncherError> {
    Ok(state.config.clone())
}

#[tauri::command]
pub fn save_config(
    state: tauri::State<'_, crate::AppState>,
    config: crate::config::LauncherConfig,
) -> Result<bool, LauncherError> {
    // Wir kÃ¶nnen den state nicht direkt mutieren, also speichern wir in die Datei
    // Die AppState.config ist im State, aber tauri::State gibt immutable Zugriff.
    // FÃ¼r MVP speichern wir direkt in die Datei
    let config_path = config.app_data_dir.join("launcher.json");
    let data = serde_json::to_string_pretty(&config).map_err(|e| LauncherError::Config(e.to_string()))?;
    std::fs::write(&config_path, data).map_err(|e| LauncherError::Config(e.to_string()))?;
    Ok(true)
}

pub fn ensure_dir(path: &Path) -> Result<(), crate::error::LauncherError> {
    fs::create_dir_all(path).map_err(|e| LauncherError::Io(e))
}

fn default_app_data_dir() -> PathBuf {
    if cfg!(target_os = "windows") {
        if let Ok(appdata) = std::env::var("APPDATA") {
            PathBuf::from(appdata).join("MinecraftLauncher")
        } else {
            dirs_home().join(".minecraftlauncher")
        }
    } else if cfg!(target_os = "macos") {
        dirs_home().join("Library/Application Support/MinecraftLauncher")
    } else {
        if let Ok(xdg) = std::env::var("XDG_DATA_HOME") {
            PathBuf::from(xdg).join("MinecraftLauncher")
        } else {
            dirs_home().join(".local/share/MinecraftLauncher")
        }
    }
}

fn dirs_home() -> PathBuf {
    std::env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."))
}

