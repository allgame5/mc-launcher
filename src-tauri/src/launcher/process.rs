use crate::error::LauncherError;
use crate::instances::model::InstanceJson;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::BufRead;
use std::process::{Command, Stdio};
use tauri::Emitter;
use tauri::State;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchRequest {
    pub instance_id: String,
    pub java_path: Option<String>,
    pub ram_mb: Option<u32>,
}

#[derive(Debug, Serialize)]
#[serde(tag = "event", content = "payload")]
pub enum LauncherEvent {
    Log { line: String },
    Started { pid: u32 },
    Stopped { exit_code: Option<i32> },
    Error { message: String },
}

#[tauri::command]
pub async fn launch_minecraft(
    app_handle: tauri::AppHandle,
    state: State<'_, crate::AppState>,
    request: LaunchRequest,
) -> Result<(), LauncherError> {
    let config = &state.config;
    let instance_dir = config.instance_dir(&request.instance_id);
    let json_path = instance_dir.join("instance.json");

    if !json_path.exists() {
        return Err(LauncherError::InstanceNotFound(request.instance_id.clone()));
    }

    let data = fs::read_to_string(&json_path).map_err(|e| LauncherError::Io(e.to_string()))?;
    let instance_json: InstanceJson =
        serde_json::from_str(&data).map_err(|e| LauncherError::Json(e.to_string()))?;

    // Java-Path bestimmen
    let java_path = request
        .java_path
        .or_else(|| config.java_path.clone().map(|p| p.to_string_lossy().to_string()))
        .unwrap_or_else(|| {
            let major_dir = format!("jre-{}-latest", crate::launcher::java::java_major_for_mc(&instance_json.mc_version));
            config.paths().java_dir.join(&major_dir).join("bin").join("java.exe").to_string_lossy().to_string()
        });

    let ram_mb = request.ram_mb.unwrap_or(instance_json.ram_mb);

    // Version JSON laden
    let versions_dir = instance_dir.join("versions");
    let version_json_path = versions_dir.join(format!("{}.json", instance_json.mc_version));
    if !version_json_path.exists() {
        return Err(LauncherError::DownloadFailed(format!(
            "Version JSON nicht gefunden für {}. Bitte zuerst herunterladen.",
            instance_json.mc_version
        )));
    }
    let version_data = fs::read_to_string(&version_json_path).map_err(|e| LauncherError::Io(e.to_string()))?;
    let version_json: crate::launcher::manifest::VersionJson =
        serde_json::from_str(&version_data).map_err(|e| LauncherError::Json(e.to_string()))?;

    // Classpath aufbauen
    let libs_dir = instance_dir.join("libraries");
    let mut classpath_entries: Vec<String> = Vec::new();

    // Client-JAR
    let client_jar = instance_dir.join("bin").join(format!("{}.jar", instance_json.mc_version));
    if client_jar.exists() {
        classpath_entries.push(client_jar.to_string_lossy().to_string());
    }

    // Libraries
    for lib in &version_json.libraries {
        if !crate::launcher::manifest::library_applies(lib) {
            continue;
        }
        if let Some(downloads) = &lib.downloads {
            if let Some(artifact) = &downloads.artifact {
                let path = artifact
                    .path
                    .as_deref()
                    .unwrap_or_else(|| lib.name.replace(':', "/").as_str());
                let lib_path = libs_dir.join(path);
                if lib_path.exists() {
                    classpath_entries.push(lib_path.to_string_lossy().to_string());
                }
            }
            // Natives
            if let Some(classifiers) = &downloads.classifiers {
                if let Some(classifier) = crate::launcher::manifest::library_native_classifier(lib) {
                    if let Some(native) = classifiers.get(&classifier) {
                        let path = native
                            .get("path")
                            .and_then(|v| v.as_str())
                            .unwrap_or_else(|| format!("{}-{}", lib.name, classifier).as_str());
                        let native_path = libs_dir.join(path);
                        if native_path.exists() {
                            classpath_entries.push(native_path.to_string_lossy().to_string());
                        }
                    }
                }
            }
        }
    }

    let classpath = classpath_entries.join(if cfg!(target_os = "windows") { ";" } else { ":" });

    // Natives-Verzeichnis
    let natives_dir = instance_dir.join("natives");
    fs::create_dir_all(&natives_dir).map_err(|e| LauncherError::Io(e.to_string()))?;

    // JVM-Argumente
    let mut jvm_args: Vec<String> = Vec::new();
    jvm_args.push(format!("-Xmx{}M", ram_mb));
    jvm_args.push(format!("-Xms{}M", ram_mb / 2));
    jvm_args.push("-Djava.library.path=".to_string() + &natives_dir.to_string_lossy());

    // Standard Minecraft JVM-Args (vereinfacht)
    jvm_args.push("-Dminecraft.launcher.launcher=tauri-launcher".to_string());

    // Hauptklasse
    let main_class = &version_json.main_class;

    // Minecraft-Argumente (vereinfacht, Platzhalter werden ersetzt)
    let mc_args = version_json
        .minecraft_arguments
        .unwrap_or_else(|| "--username ${auth_player_name} --uuid ${auth_uuid} --token ${auth_access_token} --version ${version_name} --gameDir ${game_directory} --assetsDir ${assets_root} --assetIndex ${assets_index_name} --uuid ${auth_uuid} --userType ${user_type} --versionType ${version_type}".to_string());

    // Befehl zusammenbauen
    let mut cmd = Command::new(&java_path);
    cmd.current_dir(&instance_dir);
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    for arg in &jvm_args {
        cmd.arg(arg);
    }
    cmd.arg("-cp");
    cmd.arg(&classpath);
    cmd.arg(main_class);

    // Minecraft-Args einfügen (vereinfacht, ohne Auth-Tokens für MVP)
    let mc_args_filled = mc_args
        .replace("${auth_player_name}", "Player")
        .replace("${auth_uuid}", "00000000-0000-0000-000000000000")
        .replace("${auth_access_token}", "")
        .replace("${version_name}", &instance_json.mc_version)
        .replace("${game_directory}", &instance_dir.to_string_lossy())
        .replace("${assets_root}", &instance_dir.join("assets").to_string_lossy())
        .replace("${assets_index_name}", &version_json.asset_index.id)
        .replace("${auth_uuid}", "00000000-0000-0000-000000000000")
        .replace("${user_type}", "legacy")
        .replace("${version_type}", "custom");

    for part in shell_words::split(&mc_args_filled).unwrap_or_default() {
        cmd.arg(part);
    }

    // Log-Event senden
    let emit_log = move |line: String| {
        let _ = app_handle.emit("launcher:log", LauncherEvent::Log { line });
    };

    let emit_started = move |pid: u32| {
        let _ = app_handle.emit("launcher:started", LauncherEvent::Started { pid });
    };

    let emit_stopped = move |code: Option<i32>| {
        let _ = app_handle.emit("launcher:stopped", LauncherEvent::Stopped { exit_code: code });
    };

    let emit_error = move |msg: String| {
        let _ = app_handle.emit("launcher:error", LauncherEvent::Error { message: msg });
    };

    // Prozess in tokio task starten
    tokio::spawn(async move {
        let mut child = match cmd.spawn() {
            Ok(c) => c,
            Err(e) => {
                emit_error(format!("Fehler beim Starten: {}", e));
                return;
            }
        };

        let pid = child.id();
        emit_started(pid);

        // Stdout lesen
        if let Some(stdout) = child.stdout.take() {
            let stdout_reader = std::io::BufReader::new(stdout);
            let emit_log_clone = emit_log;
            tokio::spawn(async move {
                for line in stdout_reader.lines() {
                    if let Ok(l) = line {
                        emit_log_clone(format!("[STDOUT] {}", l));
                    }
                }
            });
        }

        // Stderr lesen
        if let Some(stderr) = child.stderr.take() {
            let stderr_reader = std::io::BufReader::new(stderr);
            let emit_log_clone = emit_log;
            tokio::spawn(async move {
                for line in stderr_reader.lines() {
                    if let Ok(l) = line {
                        emit_log_clone(format!("[STDERR] {}", l));
                    }
                }
            });
        }

        let exit_status = child.wait();
        match exit_status {
            Ok(status) => emit_stopped(status.code()),
            Err(e) => {
                emit_error(format!("Fehler beim Warten auf Prozess: {}", e));
                emit_stopped(None);
            }
        }
    });

    Ok(())
}

// shell_words-Crate wäre hierfür ideal, aber wir machen es einfach ohne für MVP
mod shell_words {
    pub fn split(s: &str) -> Option<Vec<String>> {
        let mut args = Vec::new();
        let mut current = String::new();
        let mut in_quotes = false;
        let mut chars = s.chars().peekable();

        while let Some(c) = chars.next() {
            if c == '"' {
                in_quotes = !in_quotes;
            } else if c == ' ' && !in_quotes {
                if !current.is_empty() {
                    args.push(current.clone());
                    current.clear();
                }
            } else {
                current.push(c);
            }
        }

        if !current.is_empty() {
            args.push(current);
        }

        Some(args)
    }
}
