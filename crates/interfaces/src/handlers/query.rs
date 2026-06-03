use axum::{
    extract::{Path, Query, State},
    Json,
};
use sql_admin_application::dto::{ExecuteQueryRequest, GetTableDataRequest};
use sql_admin_api_types::ApiResponse;

use crate::error::AppResult;
use crate::state::AppState;

pub async fn execute_query(
    State(state): State<AppState>,
    Json(cmd): Json<ExecuteQueryRequest>,
) -> AppResult<Json<ApiResponse<sql_admin_domain::shared::pool::QueryResult>>> {
    let result = state.query_handler.execute(cmd).await?;
    Ok(Json(ApiResponse::ok(result)))
}

#[derive(serde::Deserialize)]
pub struct TableDataParams {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

pub async fn get_table_data(
    State(state): State<AppState>,
    Path((conn_id, table)): Path<(String, String)>,
    Query(params): Query<TableDataParams>,
) -> AppResult<Json<ApiResponse<sql_admin_domain::shared::pool::QueryResult>>> {
    let cmd = GetTableDataRequest {
        connection_id: conn_id,
        table,
        limit: params.limit.unwrap_or(100),
        offset: params.offset.unwrap_or(0),
    };
    let result = state.query_handler.get_table_data(cmd).await?;
    Ok(Json(ApiResponse::ok(result)))
}
