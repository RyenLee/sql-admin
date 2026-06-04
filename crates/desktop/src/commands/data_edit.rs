use tauri::State;
use sql_admin_api_types::*;
use crate::state::AppState;

fn rows_affected_to_query_result(rows: u64) -> QueryResult {
    QueryResult {
        columns: vec![],
        rows: vec![],
        rows_affected: Some(rows),
        execution_time_ms: None,
    }
}

#[tauri::command]
pub async fn edit_row(
    state: State<'_, AppState>,
    connection_id: String,
    mut request: EditRowRequest,
) -> Result<QueryResult, String> {
    request.connection_id = connection_id;
    let rows = state.data_edit_handler
        .edit_row(request)
        .await
        .map_err(|e| e.to_string())?;
    Ok(rows_affected_to_query_result(rows))
}

#[tauri::command]
pub async fn delete_row(
    state: State<'_, AppState>,
    connection_id: String,
    mut request: DeleteRowRequest,
) -> Result<QueryResult, String> {
    request.connection_id = connection_id;
    let rows = state.data_edit_handler
        .delete_row(request)
        .await
        .map_err(|e| e.to_string())?;
    Ok(rows_affected_to_query_result(rows))
}

#[tauri::command]
pub async fn insert_row(
    state: State<'_, AppState>,
    connection_id: String,
    mut request: InsertRowRequest,
) -> Result<QueryResult, String> {
    request.connection_id = connection_id;
    let rows = state.data_edit_handler
        .insert_row(request)
        .await
        .map_err(|e| e.to_string())?;
    Ok(rows_affected_to_query_result(rows))
}
