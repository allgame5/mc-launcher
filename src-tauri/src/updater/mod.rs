use tauri::Emitter;
use tauri_plugin_updater::UpdaterExt;

#[tauri::command]
pub async fn check_for_updates(
    app_handle: tauri::AppHandle,
) -> Result<bool, String> {
    let updater = app_handle.updater().map_err(|e| e.to_string())?;
    let update = updater.check().await.map_err(|e| e.to_string())?;

    if let Some(update) = update {
        let _ = app_handle.emit("updater:available", serde_json::json!({
            "version": update.version,
            "body": update.body,
            "url": update.url,
        }));

        Ok(true)
    } else {
        Ok(false)
    }
}
