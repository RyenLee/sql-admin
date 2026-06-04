use tauri::State;
use sql_admin_api_types::*;
use sql_admin_application::dto::CreateConnectionRequest;
use crate::state::AppState;

#[tauri::command]
pub async fn list_connections(
    state: State<'_, AppState>,
) -> Result<Vec<Connection>, String> {
    state.connection_handler
        .list()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_connection(
    state: State<'_, AppState>,
    request: CreateConnectionRequest,
) -> Result<Connection, String> {
    state.connection_handler
        .create(request)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_connection(
    state: State<'_, AppState>,
    id: String,
) -> Result<Connection, String> {
    state.connection_handler
        .get_by_id(&id)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Connection not found".to_string())
}

#[tauri::command]
pub async fn update_connection(
    state: State<'_, AppState>,
    id: String,
    request: UpdateConnectionRequest,
) -> Result<Connection, String> {
    state.connection_handler
        .update(id, request)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_connection(
    state: State<'_, AppState>,
    id: String,
) -> Result<bool, String> {
    let cmd = DeleteConnectionRequest { id };
    state.connection_handler
        .delete(cmd)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn test_connection(
    state: State<'_, AppState>,
    id: String,
) -> Result<String, String> {
    state.connection_handler
        .test_connection(&id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn test_connection_request(
    state: State<'_, AppState>,
    request: CreateConnectionRequest,
) -> Result<String, String> {
    state.connection_handler
        .test_connection_request(request)
        .await
        .map_err(|e| e.to_string())
}
