use axum::{
    extract::{Path, State},
    Json,
};
use sql_admin_application::dto::{
    Connection, CreateConnectionRequest, DeleteConnectionRequest, UpdateConnectionRequest,
};
use sql_admin_api_types::ApiResponse;

use crate::error::AppResult;
use crate::state::AppState;

pub async fn create_connection(
    State(state): State<AppState>,
    Json(req): Json<CreateConnectionRequest>,
) -> AppResult<Json<ApiResponse<Connection>>> {
    let result = state.connection_handler.create(req).await?;
    Ok(Json(ApiResponse::ok(result)))
}

pub async fn list_connections(
    State(state): State<AppState>,
) -> AppResult<Json<ApiResponse<Vec<Connection>>>> {
    let result = state.connection_handler.list().await?;
    Ok(Json(ApiResponse::ok(result)))
}

pub async fn get_connection(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> AppResult<Json<ApiResponse<Connection>>> {
    let result = state
        .connection_handler
        .get_by_id(&id)
        .await?
        .ok_or_else(|| {
            sql_admin_domain::shared::application_error::ApplicationError::NotFound(format!(
                "Connection not found: {}",
                id
            ))
        })?;
    Ok(Json(ApiResponse::ok(result)))
}

pub async fn update_connection(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<UpdateConnectionRequest>,
) -> AppResult<Json<ApiResponse<Connection>>> {
    let result = state.connection_handler.update(id, req).await?;
    Ok(Json(ApiResponse::ok(result)))
}

pub async fn delete_connection(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> AppResult<Json<ApiResponse<bool>>> {
    let cmd = DeleteConnectionRequest { id };
    let result = state.connection_handler.delete(cmd).await?;
    Ok(Json(ApiResponse::ok(result)))
}

pub async fn test_connection(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> AppResult<Json<ApiResponse<String>>> {
    let result = state.connection_handler.test_connection(&id).await?;
    Ok(Json(ApiResponse::ok(result)))
}

pub async fn test_connection_request(
    State(state): State<AppState>,
    Json(req): Json<CreateConnectionRequest>,
) -> AppResult<Json<ApiResponse<String>>> {
    let result = state.connection_handler.test_connection_request(req).await?;
    Ok(Json(ApiResponse::ok(result)))
}
