use crate::error::LauncherError;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha512};
use std::fs;
use std::io::Write;
use std::time::Duration;

const BASE_URL: &str = "https://api.modrinth.com/v2";
const USER_AGENT: &str = "MinecraftLauncher/0.1.0 (https://github.com/mclauncher/app; contact@example.com)";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModrinthSearchResult {
    pub hits: Vec<ModrinthProject>,
    pub total: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModrinthProject {
    pub id: String,
    pub title: String,
    pub slug: String,
    pub description: String,
    pub project_type: String,
    pub icon_url: Option<String>,
    pub downloads: u64,
    pub followers: u64,
    pub categories: Vec<String>,
    pub versions: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModrinthVersion {
    pub id: String,
    pub project_id: String,
    pub version_number: String,
    pub game_versions: Vec<String>,
    pub loaders: Vec<String>,
    pub files: Vec<ModrinthFile>,
    pub dependencies: Vec<ModrinthDependency>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModrinthFile {
    pub name: String,
    pub url: String,
    pub filename: String,
    pub size: usize,
    pub hashes: ModrinthHashes,
    pub primary: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModrinthHashes {
    pub sha512: String,
    pub sha1: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModrinthDependency {
    pub project_id: String,
    pub version_id: String,
    pub dependency_type: String,
}

pub struct ModrinthClient {
    client: Client,
}

impl ModrinthClient {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .expect("failed to create HTTP client"),
        }
    }

    pub async fn search(
        &self,
        query: &str,
        limit: usize,
        offset: usize,
        filters: Option<&ModrinthSearchFilters>,
    ) -> Result<ModrinthSearchResult, LauncherError> {
        let mut url = format!("{}/search?query={}&limit={}&offset={}", BASE_URL, urlencoding::encode(query), limit, offset);

        if let Some(filters) = filters {
            if let Some(loader) = &filters.loader {
                url.push_str(&format!("&facets=[[\"loader:{}\"]]", loader));
            }
            if let Some(version) = &filters.game_version {
                url.push_str(&format!("&facets=[[\"game_version:{}\"]]", version));
            }
            if let Some(project_type) = &filters.project_type {
                url.push_str(&format!("&facets=[[\"project_type:{}\"]]", project_type));
            }
        }

        let resp = self
            .client
            .get(&url)
            .header("User-Agent", USER_AGENT)
            .send()
            .await
            .map_err(|e| LauncherError::Http(e))?;

        let result: ModrinthSearchResult = resp
            .json()
            .await
            .map_err(|e| LauncherError::Http(e))?;

        Ok(result)
    }

    pub async fn get_project_versions(
        &self,
        project_id: &str,
        game_version: Option<&str>,
        loader: Option<&str>,
    ) -> Result<Vec<ModrinthVersion>, LauncherError> {
        let mut url = format!("{}/project/{}/version", BASE_URL, project_id);
        let mut facets = Vec::new();
        if let Some(gv) = game_version {
            facets.push(format!("game_version:{}", gv));
        }
        if let Some(l) = loader {
            facets.push(format!("loader:{}", l));
        }
        if !facets.is_empty() {
            url.push_str(&format!("?facets=[{}]", facets.join(",")));
        }

        let resp = self
            .client
            .get(&url)
            .header("User-Agent", USER_AGENT)
            .send()
            .await
            .map_err(|e| LauncherError::Http(e))?;

        let versions: Vec<ModrinthVersion> = resp
            .json()
            .await
            .map_err(|e| LauncherError::Http(e))?;

        Ok(versions)
    }

    pub async fn download_file(&self, url: &str, dest: &std::path::Path, expected_sha512: &str) -> Result<(), LauncherError> {
        let mut resp = self
            .client
            .get(url)
            .header("User-Agent", USER_AGENT)
            .send()
            .await
            .map_err(|e| LauncherError::Http(e))?;

        if let Some(parent) = dest.parent() {
            std::fs::create_dir_all(parent).map_err(|e| LauncherError::Io(e))?;
        }

        let mut file = std::fs::File::create(dest).map_err(|e| LauncherError::Io(e))?;
        while let Some(chunk) = resp.chunk().await.map_err(|e| LauncherError::Http(e))? {
            file.write_all(&chunk).map_err(|e| LauncherError::Io(e))?;
        }

        let data = std::fs::read(dest).map_err(|e| LauncherError::Io(e))?;
        let mut hasher = Sha512::new();
        hasher.update(&data);
        let actual = format!("{:x}", hasher.finalize());

        if actual != expected_sha512 {
            return Err(LauncherError::HashMismatch {
                expected: expected_sha512.to_string(),
                actual,
            });
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModrinthSearchFilters {
    pub loader: Option<String>,
    pub game_version: Option<String>,
    pub project_type: Option<String>,
}
