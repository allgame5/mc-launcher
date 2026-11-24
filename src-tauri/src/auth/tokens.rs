use crate::error::LauncherError;
use crate::auth::device_code::{load_accounts, save_accounts, StoredAccount};
use serde::Deserialize;
use tauri::State;

#[derive(Debug, Deserialize)]
pub struct LogoutRequest {
    pub username: String,
}

#[derive(Debug, Deserialize)]
pub struct SetActiveRequest {
    pub username: String,
}

#[tauri::command]
pub async fn logout_account(
    state: State<'_, crate::AppState>,
    request: LogoutRequest,
) -> Result<bool, LauncherError> {
    let config = &state.config;
    let mut accounts = load_accounts(config);
    accounts.retain(|a| a.username != request.username);
    save_accounts(config, &accounts);
    Ok(true)
}

#[tauri::command]
pub async fn set_active_account(
    state: State<'_, crate::AppState>,
    request: SetActiveRequest,
) -> Result<bool, LauncherError> {
    let config = &state.config;
    let mut accounts = load_accounts(config);

    for acc in &mut accounts {
        acc.is_active = acc.username == request.username;
    }

    save_accounts(config, &accounts);
    Ok(true)
}
