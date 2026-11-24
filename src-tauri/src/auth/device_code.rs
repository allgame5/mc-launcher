use crate::config::LauncherConfig;
use crate::error::LauncherError;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::State;

const DEVICE_CODE_URL: &str = "https://login.microsoftonline.com/common/oauth2/v2.0/devicecode";
const TOKEN_URL: &str = "https://login.microsoftonline.com/common/oauth2/v2.0/token";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceCodeResponse {
    pub user_code: String,
    pub device_code: String,
    pub verification_uri: String,
    pub expires_in: i64,
    pub interval: i64,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
    pub token_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredAccount {
    pub username: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: Option<i64>,
    pub is_active: bool,
    pub xuid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XblResult {
    pub user: String,
    pub xuid: String,
    pub minecraft_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthRequest {
    pub client_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PollRequest {
    pub device_code: String,
    pub client_id: Option<String>,
}

pub fn accounts_path(config: &LauncherConfig) -> PathBuf {
    config.app_data_dir.join("accounts.json")
}

pub fn load_accounts(config: &LauncherConfig) -> Vec<StoredAccount> {
    let path = accounts_path(config);
    if !path.exists() {
        return Vec::new();
    }
    let data = match fs::read_to_string(&path) {
        Ok(d) => d,
        Err(_) => return Vec::new(),
    };
    serde_json::from_str(&data).unwrap_or_default()
}

pub fn save_accounts(config: &LauncherConfig, accounts: &[StoredAccount]) {
    let path = accounts_path(config);
    let _ = fs::write(
        &path,
        serde_json::to_string_pretty(accounts).unwrap_or_default(),
    );
}

fn get_client_id(config: &LauncherConfig, override_id: Option<&str>) -> Result<String, LauncherError> {
    if let Some(id) = override_id {
        if !id.is_empty() {
            return Ok(id.to_string());
        }
    }
    if let Some(ref id) = config.client_id {
        return Ok(id.clone());
    }
    Err(LauncherError::Auth(
        "Keine Microsoft Client-ID konfiguriert. Setze MINECRAFT_LAUNCHER_CLIENT_ID oder in den Einstellungen.".to_string(),
    ))
}

#[tauri::command]
pub async fn authenticate_microsoft(
    app_handle: tauri::AppHandle,
    state: State<'_, crate::AppState>,
    request: AuthRequest,
) -> Result<DeviceCodeResponse, LauncherError> {
    let config = &state.config;
    let client_id = get_client_id(config, request.client_id.as_deref())?;

    let client = Client::new();
    let params = [
        ("client_id", client_id.as_str()),
        ("scope", "XboxLive.signin XboxLive.offline_access"),
    ];

    let resp: DeviceCodeResponse = client
        .post(DEVICE_CODE_URL)
        .form(&params)
        .header("User-Agent", "MinecraftLauncher/0.1.0")
        .send()
        .await
        .map_err(|e| LauncherError::Http(e))?
        .json()
        .await
        .map_err(|e| LauncherError::Json(e))?;

    Ok(resp)
}

#[tauri::command]
pub async fn device_code_poll(
    app_handle: tauri::AppHandle,
    state: State<'_, crate::AppState>,
    request: PollRequest,
) -> Result<StoredAccount, LauncherError> {
    let config = &state.config;
    let client_id = get_client_id(config, request.client_id.as_deref())?;

    let client = Client::new();

    // Bis zu 10 Minuten pollen (alle interval Sekunden)
    let mut attempts = 0;
    let max_attempts = 600; // ~10 Minuten bei 1s Intervall

    loop {
        let params = [
            ("client_id", client_id.as_str()),
            ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
            ("device_code", &request.device_code),
        ];

        let resp: Result<TokenResponse, serde_json::Value> = client
            .post(TOKEN_URL)
            .form(&params)
            .header("User-Agent", "MinecraftLauncher/0.1.0")
            .send()
            .await
            .map_err(|e| LauncherError::Http(e))?
            .json()
            .await
            .map_err(|e| LauncherError::Json(e));

        match resp {
            Ok(tokens) => {
                // Token erhalten - nun XBL Auth
                let xbl_result = xbl_authenticate(tokens.access_token.clone()).await?;
                let username = xbl_result.user.clone().unwrap_or_else(|| "Unknown".to_string());
                let xuid = xbl_result.xuid.clone();

                let mut accounts = load_accounts(config);

                // Bestehenden Account mit gleichem Benutzernamen aktualisieren oder neu hinzufügen
                if let Some(existing) = accounts.iter_mut().find(|a| a.username == username) {
                    existing.access_token = tokens.access_token;
                    existing.refresh_token = tokens.refresh_token;
                    existing.expires_at = Some(chrono::Utc::now().timestamp() + tokens.expires_in);
                    existing.xuid = xuid;
                } else {
                    accounts.push(StoredAccount {
                        username,
                        access_token: tokens.access_token,
                        refresh_token: tokens.refresh_token,
                        expires_at: Some(chrono::Utc::now().timestamp() + tokens.expires_in),
                        is_active: accounts.is_empty(),
                        xuid,
                    });
                }

                save_accounts(config, &accounts);

                return Ok(accounts.last().unwrap().clone());
            }
            Err(err_value) => {
                // Prüfe auf authorization_pending
                let error_code = err_value.get("error").and_then(|v| v.as_str()).unwrap_or("");
                if error_code == "authorization_pending" {
                    attempts += 1;
                    if attempts > max_attempts {
                        return Err(LauncherError::Auth("Device-Code abgelaufen".to_string()));
                    }
                    // Warte interval Sekunden
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                    continue;
                } else if error_code == "slow_down" {
                    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                    continue;
                } else {
                    return Err(LauncherError::Auth(format!("Auth-Fehler: {:?}", err_value)));
                }
            }
        }
    }
}

#[tauri::command]
pub async fn get_stored_accounts(
    state: State<'_, crate::AppState>,
) -> Result<Vec<StoredAccount>, LauncherError> {
    let config = &state.config;
    Ok(load_accounts(config))
}
