use crate::error::LauncherError;
use crate::modrinth::client::{ModrinthClient, ModrinthSearchFilters};
use serde::Deserialize;
use tauri::State;

#[derive(Debug, Deserialize)]
pub struct SearchRequest {
    pub query: String,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub loader: Option<String>,
    pub game_version: Option<String>,
    pub project_type: Option<String>,
}

#[tauri::command]
pub async fn search_modrinth(
    state: State<'_, crate::AppState>,
    request: SearchRequest,
) -> Result<crate::modrinth::client::ModrinthSearchResult, LauncherError> {
    let client = ModrinthClient::new();
    let filters = ModrinthSearchFilters {
        loader: request.loader,
        game_version: request.game_version,
        project_type: request.project_type,
    };

    let result = client
        .search(
            &request.query,
            request.limit.unwrap_or(20),
            request.offset.unwrap_or(0),
            Some(&filters),
        )
        .await?;

    Ok(result)
}
