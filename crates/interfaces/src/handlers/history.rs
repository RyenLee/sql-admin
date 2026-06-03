use axum::{
    extract::{Path, State},
    Json,
};
use sql_admin_application::dto::{QueryHistory, SaveQueryHistoryRequest};
use sql_admin_api_types::ApiResponse;

use crate::error::AppResult;
use crate::state::AppState;

pub async fn save_query_history(
    State(state): State<AppState>,
    Json(cmd): Json<SaveQueryHistoryRequest>,
) -> AppResult<Json<ApiResponse<QueryHistory>>> {
    let result = state.history_handler.save(cmd).await?;
    Ok(Json(ApiResponse::ok(result)))
}

pub async fn get_query_history(
    State(state): State<AppState>,
) -> AppResult<Json<ApiResponse<Vec<QueryHistory>>>> {
    let result = state.history_handler.list().await?;
    Ok(Json(ApiResponse::ok(result)))
}

pub async fn delete_query_history(
    State(state): State<AppState>,
) -> AppResult<Json<ApiResponse<()>>> {
    state.history_handler.delete_all().await?;
    Ok(Json(ApiResponse::ok(())))
}

pub async fn delete_query_history_item(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> AppResult<Json<ApiResponse<bool>>> {
    let result = state.history_handler.delete_by_id(&id).await?;
    Ok(Json(ApiResponse::ok(result)))
}
