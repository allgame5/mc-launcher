#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod auth;
mod config;
mod error;
mod instances;
mod launcher;
mod modrinth;
mod updater;

use auth::{
    authenticate_microsoft, device_code_poll, get_stored_accounts, logout_account,
    set_active_account,
};
use config::{ensure_dir, LauncherConfig};
#[allow(unused_imports)]
use error::LauncherError;
use instances::{
    create_instance, delete_instance, edit_instance, get_instance, list_instances,
};
use launcher::{download_instance, launch_minecraft, get_java_path, verify_instance_files};
use modrinth::{
    install_modrinth_project, list_installed_mods, search_modrinth, update_installed_mods,
};
use updater::check_for_updates;

pub struct AppState {
    pub config: LauncherConfig,
}

fn main() {
    // Config vor dem Builder laden, damit wir sie manage()n kÃ¶nnen
    let config = LauncherConfig::load_raw().expect("failed to load launcher config");

    let app_data_dir = config.paths().instances_dir.parent().unwrap().to_path_buf();

    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::default().build())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .manage(AppState { config })
        .setup(move |app| {
            let app_handle = app.handle();
            let _ = app_data_dir.clone();

            ensure_dir(&app_data_dir).map_err(|e| {
                std::io::Error::other(e.to_string()).into()
            })?;
            ensure_dir(&app_data_dir.join("instances")).map_err(|e| {
                std::io::Error::other(e.to_string()).into()
            })?;
            ensure_dir(&app_data_dir.join("java")).map_err(|e| {
                std::io::Error::other(e.to_string()).into()
            })?;
            ensure_dir(&app_data_dir.join("cache")).map_err(|e| {
                std::io::Error::other(e.to_string()).into()
            })?;
            ensure_dir(&app_data_dir.join("logs")).map_err(|e| {
                std::io::Error::other(e.to_string()).into()
            })?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Config
            config::get_config,
            config::save_config,
            // Instances
            list_instances,
            create_instance,
            get_instance,
            edit_instance,
            delete_instance,
            // Launcher
            download_instance,
            launch_minecraft,
            get_java_path,
            verify_instance_files,
            // Modrinth
            search_modrinth,
            install_modrinth_project,
            list_installed_mods,
            update_installed_mods,
            // Auth
            authenticate_microsoft,
            device_code_poll,
            get_stored_accounts,
            logout_account,
            set_active_account,
            // Updater
            check_for_updates,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

