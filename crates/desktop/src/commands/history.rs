use tauri::State;
use sql_admin_api_types::*;
use sql_admin_application::dto::SaveQueryHistoryRequest;
use crate::state::AppState;

#[tauri::command]
pub async fn get_query_history(
    state: State<'_, AppState>,
) -> Result<Vec<QueryHistory>, String> {
    state.history_handler
        .list()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn save_query_history(
    state: State<'_, AppState>,
    request: SaveQueryHistoryRequest,
) -> Result<QueryHistory, String> {
    state.history_handler
        .save(request)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn clear_query_history(
    state: State<'_, AppState>,
) -> Result<(), String> {
    state.history_handler
        .delete_all()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_query_history_item(
    state: State<'_, AppState>,
    id: String,
) -> Result<bool, String> {
    state.history_handler
        .delete_by_id(&id)
        .await
        .map_err(|e| e.to_string())
}
