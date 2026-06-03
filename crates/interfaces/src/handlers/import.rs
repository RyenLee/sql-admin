use axum::{
    extract::{Path, State},
    Json,
};
use sql_admin_application::dto::{ImportResult, ImportSqlRequest};
use sql_admin_api_types::ApiResponse;

use crate::error::AppResult;
use crate::state::AppState;

pub async fn import_sql(
    State(state): State<AppState>,
    Path(conn_id): Path<String>,
    Json(mut cmd): Json<ImportSqlRequest>,
) -> AppResult<Json<ApiResponse<ImportResult>>> {
    cmd.connection_id = conn_id;
    let result = state.import_handler.import_sql(cmd).await?;
    Ok(Json(ApiResponse::ok(result)))
}
