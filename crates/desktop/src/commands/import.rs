use tauri::State;
use sql_admin_application::dto::*;
use crate::state::AppState;

#[tauri::command]
pub async fn import_sql(
    state: State<'_, AppState>,
    connection_id: String,
    mut request: ImportSqlRequest,
) -> Result<ImportResult, String> {
    request.connection_id = connection_id;
    state.import_handler
        .import_sql(request)
        .await
        .map_err(|e| e.to_string())
}
