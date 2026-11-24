use tauri::{Manager, State};

#[tauri::command]
pub async fn check_for_updates(
    app_handle: tauri::AppHandle,
    _state: State<'_, crate::AppState>,
) -> Result<bool, String> {
    let updater = app_handle.updater();
    let update = updater.check().await.map_err(|e| e.to_string())?;

    if let Some(update) = update {
        // Update verfügbar - Event ans Frontend senden
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
