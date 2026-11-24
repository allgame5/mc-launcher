use crate::error::LauncherError;
use reqwest::Client;
use std::fs;
use std::io::Write;
use std::path::Path;

pub fn java_major_for_mc(mc_version: &str) -> i32 {
    let parts: Vec<&str> = mc_version.split('.').collect();
    let major: i32 = parts.get(0).and_then(|p| p.parse().ok()).unwrap_or(1);
    let minor: i32 = parts.get(1).and_then(|p| p.parse().ok()).unwrap_or(0);

    if major == 1 && minor <= 16 {
        8
    } else if major == 1 && minor >= 21 {
        21
    } else if major == 1 {
        17
    } else if major >= 2 {
        21
    } else {
        17
    }
}

fn adoptium_api_url(major: i32) -> String {
    format!(
        "https://api.adoptium.net/v3/meta/release?major={}&release_type=ga&heap_size=normal",
        major
    )
}

fn current_os_arch() -> (&'static str, &'static str) {
    let os = if cfg!(target_os = "windows") {
        "windows"
    } else if cfg!(target_os = "macos") {
        "mac"
    } else if cfg!(target_os = "linux") {
        "linux"
    } else {
        "windows"
    };

    let arch = if cfg!(target_arch = "x86_64") {
        "x64"
    } else if cfg!(target_arch = "aarch64") {
        "aarch64"
    } else {
        "x64"
    };

    (os, arch)
}

fn find_java_exe(java_dir: &Path) -> Option<std::path::PathBuf> {
    let candidates = if cfg!(target_os = "windows") {
        vec![
            java_dir.join("bin").join("java.exe"),
            java_dir.join("bin").join("javaw.exe"),
        ]
    } else {
        vec![java_dir.join("bin").join("java")]
    };

    candidates.into_iter().find(|p| p.exists())
}

#[tauri::command]
pub async fn get_java_path(
    state: tauri::State<'_, crate::AppState>,
    mc_version: String,
) -> Result<Option<String>, LauncherError> {
    let config = &state.config;
    let major = java_major_for_mc(&mc_version);
    let java_dir = config.paths().java_dir.join(format!("jre-{}-latest", major));

    if let Some(exe) = find_java_exe(&java_dir) {
        return Ok(Some(exe.to_string_lossy().to_string()));
    }

    if let Ok(java_home) = std::env::var("JAVA_HOME") {
        let path = std::path::PathBuf::from(java_home);
        if let Some(exe) = find_java_exe(&path) {
            return Ok(Some(exe.to_string_lossy().to_string()));
        }
    }

    if let Some(ref custom) = config.java_path {
        if Path::new(custom).exists() {
            return Ok(Some(custom.to_string_lossy().to_string()));
        }
    }

    Ok(None)
}

pub async fn ensure_java_for_version(
    config: &crate::config::LauncherConfig,
    mc_version: &str,
) -> Result<std::path::PathBuf, LauncherError> {
    let major = java_major_for_mc(mc_version);
    let java_dir = config.paths().java_dir.join(format!("jre-{}-latest", major));

    if find_java_exe(&java_dir).is_some() {
        return Ok(java_dir);
    }

    fs::create_dir_all(&java_dir).map_err(|e| LauncherError::Io(e.to_string()))?;

    let client = Client::new();
    let url = adoptium_api_url(major);
    let (os, arch) = current_os_arch();

    let resp: serde_json::Value = client
        .get(&url)
        .header("User-Agent", "MinecraftLauncher/0.1.0")
        .send()
        .await
        .map_err(|e| LauncherError::Http(e.to_string()))?
        .json()
        .await
        .map_err(|e| LauncherError::Http(e.to_string()))?;

    if let Some(binaries) = resp.get("binaries").and_then(|v| v.as_array()) {
        for binary in binaries {
            let binary_os = binary.get("metadata").and_then(|m| m.get("os")).and_then(|o| o.get("name")).and_then(|n| n.as_str()).unwrap_or("");
            let binary_arch = binary.get("metadata").and_then(|m| m.get("os")).and_then(|o| o.get("architecture")).and_then(|a| a.as_str()).unwrap_or("");
            let image_type = binary.get("metadata").and_then(|m| m.get("image_type")).and_then(|t| t.as_str()).unwrap_or("");

            let os_match = match os {
                "windows" => binary_os.eq_ignore_ascii_case("windows"),
                "mac" => binary_os.eq_ignore_ascii_case("mac"),
                "linux" => binary_os.eq_ignore_ascii_case("linux"),
                _ => false,
            };
            let arch_match = binary_arch.eq_ignore_ascii_case(arch);
            let is_jre = image_type.eq_ignore_ascii_case("jre");

            if os_match && arch_match && is_jre {
                if let Some(package) = binary.get("package").and_then(|p| p.as_object()) {
                    if let Some(link) = package.get("link").and_then(|l| l.as_str()) {
                        let dest = java_dir.join("adoptium.tar.gz");
                        download_extract_adoptium(&client, link, &dest, &java_dir).await?;
                        if find_java_exe(&java_dir).is_some() {
                            return Ok(java_dir);
                        }
                    }
                }
            }
        }
    }

    Err(LauncherError::JavaNotFound(format!(
        "Kein passendes JRE für Java {} gefunden", major
    )))
}

async fn download_extract_adoptium(
    client: &Client,
    url: &str,
    dest: &Path,
    java_dir: &Path,
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

    if dest.extension().and_then(|e| e.to_str()) == Some("gz") {
        let file = fs::File::open(dest).map_err(|e| LauncherError::Io(e.to_string()))?;
        let gz = flate2::read::GzDecoder::new(file);
        let mut archive = tar::Archive::new(gz);
        archive.unpack(java_dir).map_err(|e| LauncherError::Io(e.to_string()))?;
    }

    Ok(())
}
