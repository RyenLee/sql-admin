use tauri::State;
use sql_admin_api_types::QueryResult;
use sql_admin_application::dto::{ExecuteQueryRequest, GetTableDataRequest};
use crate::state::AppState;

fn domain_query_result_to_dto(r: sql_admin_domain::shared::pool::QueryResult) -> QueryResult {
    QueryResult {
        columns: r.columns,
        rows: r.rows,
        rows_affected: r.rows_affected,
        execution_time_ms: r.execution_time_ms,
    }
}

#[tauri::command]
pub async fn execute_query(
    state: State<'_, AppState>,
    request: ExecuteQueryRequest,
) -> Result<QueryResult, String> {
    state.query_handler
        .execute(request)
        .await
        .map(domain_query_result_to_dto)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_table_data(
    state: State<'_, AppState>,
    request: GetTableDataRequest,
) -> Result<QueryResult, String> {
    state.query_handler
        .get_table_data(request)
        .await
        .map(domain_query_result_to_dto)
        .map_err(|e| e.to_string())
}
