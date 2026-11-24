use crate::auth::device_code::XblResult;
use crate::error::LauncherError;
use reqwest::Client;
use serde::Serialize;

pub async fn xbl_authenticate(access_token: String) -> Result<XblResult, LauncherError> {
    let client = Client::new();

    // 1. Xbox Live Authenticate
    let xbl_body = serde_json::json!({
        "Properties": {
            "AuthMethod": "RPS",
            "TicketType": "Bearer",
            "Token": format!("Bearer {}", access_token),
        },
        "RelyingParty": "http://auth.xboxlive.com",
        "TokenType": "JWT",
    });

    let xbl_resp: serde_json::Value = client
        .post("https://user.auth.xboxlive.com/user/authenticate")
        .header("User-Agent", "MinecraftLauncher/0.1.0")
        .header("Content-Type", "application/json")
        .json(&xbl_body)
        .send()
        .await
        .map_err(|e| LauncherError::Http(e))?
        .json()
        .await
        .map_err(|e| LauncherError::Json(e))?;

    let xbl_token = xbl_resp
        .get("Token")
        .and_then(|t| t.as_str())
        .ok_or_else(|| LauncherError::Auth("Kein XBL Token erhalten".to_string()))?;

    let user_hash = xbl_resp
        .get("DisplayClaims")
        .and_then(|dc| dc.get("xui"))
        .and_then(|xui| xui.get(0))
        .and_then(|u| u.get("uhs"))
        .and_then(|u| u.as_str())
        .ok_or_else(|| LauncherError::Auth("Kein XBL UserHash erhalten".to_string()))?;

    // 2. XSTS Authenticate
    let xsts_body = serde_json::json!({
        "Properties": {
            "SandboxId": "RETAIL",
            "UserTokens": [xbl_token],
        },
        "RelyingParty": "rp://api.minecraftservices.com/",
        "TokenType": "JWT",
    });

    let xsts_resp: serde_json::Value = client
        .post("https://xsts.auth.xboxlive.com")
        .header("User-Agent", "MinecraftLauncher/0.1.0")
        .header("Content-Type", "application/json")
        .json(&xsts_body)
        .send()
        .await
        .map_err(|e| LauncherError::Http(e))?
        .json()
        .await
        .map_err(|e| LauncherError::Json(e))?;

    let xsts_token = xsts_resp
        .get("Token")
        .and_then(|t| t.as_str())
        .ok_or_else(|| LauncherError::Auth("Kein XSTS Token erhalten".to_string()))?;

    // 3. Minecraft Services Login
    let mc_body = serde_json::json!({
        "identityToken": format!("XBL3.0 x={};{}", user_hash, xsts_token),
    });

    let mc_resp: serde_json::Value = client
        .post("https://api.minecraftservices.com/authentication/login_with_xbox")
        .header("User-Agent", "MinecraftLauncher/0.1.0")
        .header("Content-Type", "application/json")
        .json(&mc_body)
        .send()
        .await
        .map_err(|e| LauncherError::Http(e))?
        .json()
        .await
        .map_err(|e| LauncherError::Json(e))?;

    let minecraft_token = mc_resp
        .get("access_token")
        .and_then(|t| t.as_str())
        .ok_or_else(|| LauncherError::Auth("Kein Minecraft Token erhalten".to_string()))?;

    Ok(XblResult {
        user: user_hash.to_string(),
        xuid: user_hash.to_string(),
        minecraft_token: minecraft_token.to_string(),
    })
}
