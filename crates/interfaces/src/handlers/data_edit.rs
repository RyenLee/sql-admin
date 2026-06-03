use axum::{
    extract::{Path, State},
    Json,
};
use sql_admin_application::dto::{DeleteRowRequest, EditRowRequest, InsertRowRequest};
use sql_admin_api_types::{ApiResponse, QueryResult};

use crate::error::AppResult;
use crate::state::AppState;

fn rows_affected_to_query_result(affected: u64) -> QueryResult {
    QueryResult {
        columns: vec!["rows_affected".to_string()],
        rows: vec![vec![serde_json::Value::Number(affected.into())]],
        rows_affected: Some(affected),
        execution_time_ms: None,
    }
}

pub async fn edit_row(
    State(state): State<AppState>,
    Path(conn_id): Path<String>,
    Json(mut cmd): Json<EditRowRequest>,
) -> AppResult<Json<ApiResponse<QueryResult>>> {
    cmd.connection_id = conn_id;
    let result = state.data_edit_handler.edit_row(cmd).await?;
    Ok(Json(ApiResponse::ok(rows_affected_to_query_result(result))))
}

pub async fn delete_row(
    State(state): State<AppState>,
    Path(conn_id): Path<String>,
    Json(mut cmd): Json<DeleteRowRequest>,
) -> AppResult<Json<ApiResponse<QueryResult>>> {
    cmd.connection_id = conn_id;
    let result = state.data_edit_handler.delete_row(cmd).await?;
    Ok(Json(ApiResponse::ok(rows_affected_to_query_result(result))))
}

pub async fn insert_row(
    State(state): State<AppState>,
    Path(conn_id): Path<String>,
    Json(mut cmd): Json<InsertRowRequest>,
) -> AppResult<Json<ApiResponse<QueryResult>>> {
    cmd.connection_id = conn_id;
    let result = state.data_edit_handler.insert_row(cmd).await?;
    Ok(Json(ApiResponse::ok(rows_affected_to_query_result(result))))
}