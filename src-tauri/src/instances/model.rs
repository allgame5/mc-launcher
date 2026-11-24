use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instance {
    pub id: Uuid,
    pub name: String,
    pub mc_version: String,
    pub mod_loader: ModLoader,
    pub loader_version: Option<String>,
    pub ram_mb: u32,
    pub icon: Option<PathBuf>,
    pub last_played: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModLoader {
    Vanilla,
    Fabric,
    NeoForge,
}

impl ModLoader {
    pub fn as_str(&self) -> &'static str {
        match self {
            ModLoader::Vanilla => "vanilla",
            ModLoader::Fabric => "fabric",
            ModLoader::NeoForge => "neoforge",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "vanilla" => Ok(ModLoader::Vanilla),
            "fabric" => Ok(ModLoader::Fabric),
            "neoforge" | "neoforged" => Ok(ModLoader::NeoForge),
            _ => Err(format!("Unsupported mod loader: {}", s)),
        }
    }
}

// Speicherformat für instance.json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceJson {
    pub id: String,
    pub name: String,
    pub mc_version: String,
    pub mod_loader: String,
    pub loader_version: Option<String>,
    pub ram_mb: u32,
    pub icon: Option<String>,
    pub last_played: Option<String>,
    pub created_at: String,
}

impl From<&Instance> for InstanceJson {
    fn from(inst: &Instance) -> Self {
        Self {
            id: inst.id.to_string(),
            name: inst.name.clone(),
            mc_version: inst.mc_version.clone(),
            mod_loader: inst.mod_loader.as_str().to_string(),
            loader_version: inst.loader_version.clone(),
            ram_mb: inst.ram_mb,
            icon: inst.icon.as_ref().map(|p| p.to_string_lossy().to_string()),
            last_played: inst.last_played.map(|d| d.to_rfc3339()),
            created_at: inst.created_at.to_rfc3339(),
        }
    }
}

impl TryFrom<InstanceJson> for Instance {
    type Error = String;

    fn try_from(json: InstanceJson) -> Result<Self, Self::Error> {
        Ok(Self {
            id: Uuid::parse_str(&json.id).map_err(|e| e.to_string())?,
            name: json.name,
            mc_version: json.mc_version,
            mod_loader: ModLoader::from_str(&json.mod_loader)?,
            loader_version: json.loader_version,
            ram_mb: json.ram_mb,
            icon: json.icon.map(PathBuf::from),
            last_played: json
                .last_played
                .as_deref()
                .map(|s| DateTime::parse_from_rfc3339(s).map(|d| d.with_timezone(&Utc)))
                .transpose()
                .map_err(|e| e.to_string())?,
            created_at: DateTime::parse_from_rfc3339(&json.created_at)
                .map_err(|e| e.to_string())?
                .with_timezone(&Utc),
        })
    }
}
