use crate::id_generator;
use crate::state::AppState;
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};
use chrono::Utc;
use sql_admin_shared::{ApiResponse, QueryHistory, SaveQueryHistoryRequest};
use sqlx::Row;
use std::sync::Arc;

pub async fn save_query_history(
    State(state): State<Arc<AppState>>,
    Json(req): Json<SaveQueryHistoryRequest>,
) -> (StatusCode, Json<ApiResponse<QueryHistory>>) {
    tracing::info!(
        module = "handlers::history",
        event = "save_query_history_start",
        connection_id = %req.connection_id,
        success = req.success,
        "Saving query history"
    );

    let id = id_generator::generate_id_string();
    let now = Utc::now();
    let connection_id = req.connection_id.clone();

    let result = sqlx::query(
        r#"
        INSERT INTO query_history (id, connection_id, connection_name, query_text, execution_time_ms, rows_count, success, error_message, created_at)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&id)
    .bind(req.connection_id)
    .bind(&req.connection_name)
    .bind(&req.query_text)
    .bind(req.execution_time_ms.map(|t| t as i64))
    .bind(req.rows_count)
    .bind(req.success)
    .bind(&req.error_message)
    .bind(now)
    .execute(&state.db_pool)
    .await;

    match result {
        Ok(_) => {
            let history = QueryHistory {
                id,
                connection_id,
                connection_name: req.connection_name,
                query_text: req.query_text,
                execution_time_ms: req.execution_time_ms.map(|t| t as i64),
                rows_count: req.rows_count,
                success: req.success,
                error_message: req.error_message,
                created_at: now,
            };
            tracing::info!(
                module = "handlers::history",
                event = "save_query_history_success",
                history_id = %history.id,
                "Query history saved successfully"
            );
            (StatusCode::CREATED, Json(ApiResponse::ok(history)))
        }
        Err(e) => {
            tracing::error!(
                module = "handlers::history",
                event = "save_query_history_failed",
                error = %e,
                "Failed to save query history"
            );
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::err("Failed to save query history")),
            )
        }
    }
}

pub async fn get_query_history(
    State(state): State<Arc<AppState>>,
) -> (StatusCode, Json<ApiResponse<Vec<QueryHistory>>>) {
    tracing::info!(
        module = "handlers::history",
        event = "get_query_history_start",
        "Fetching query history"
    );

    let rows = sqlx::query(
        r#"
        SELECT id, connection_id, connection_name, query_text, execution_time_ms, rows_count, success, error_message, created_at
        FROM query_history
        ORDER BY created_at DESC
        LIMIT 200
        "#,
    )
    .fetch_all(&state.db_pool)
    .await;

    match rows {
        Ok(rows) => {
            let history: Vec<QueryHistory> = rows
                .into_iter()
                .map(|row| QueryHistory {
                    id: row.get("id"),
                    connection_id: row.get("connection_id"),
                    connection_name: row.get("connection_name"),
                    query_text: row.get("query_text"),
                    execution_time_ms: row.get("execution_time_ms"),
                    rows_count: row.get("rows_count"),
                    success: row.get("success"),
                    error_message: row.get("error_message"),
                    created_at: row.get("created_at"),
                })
                .collect();
            tracing::info!(
                module = "handlers::history",
                event = "get_query_history_success",
                count = history.len(),
                "Query history fetched successfully"
            );
            (StatusCode::OK, Json(ApiResponse::ok(history)))
        }
        Err(e) => {
            tracing::error!(
                module = "handlers::history",
                event = "get_query_history_failed",
                error = %e,
                "Failed to get query history"
            );
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::err("Failed to get query history")),
            )
        }
    }
}

pub async fn delete_query_history(
    State(state): State<Arc<AppState>>,
) -> (StatusCode, Json<ApiResponse<String>>) {
    tracing::info!(
        module = "handlers::history",
        event = "delete_query_history_start",
        "Clearing all query history"
    );

    match sqlx::query("DELETE FROM query_history")
        .execute(&state.db_pool)
        .await
    {
        Ok(_) => {
            tracing::info!(
                module = "handlers::history",
                event = "delete_query_history_success",
                "All query history cleared"
            );
            (
                StatusCode::OK,
                Json(ApiResponse::ok("History cleared".to_string())),
            )
        }
        Err(e) => {
            tracing::error!(
                module = "handlers::history",
                event = "delete_query_history_failed",
                error = %e,
                "Failed to clear query history"
            );
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::err("Failed to clear query history")),
            )
        }
    }
}

pub async fn delete_query_history_item(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> (StatusCode, Json<ApiResponse<String>>) {
    tracing::info!(
        module = "handlers::history",
        event = "delete_query_history_item_start",
        history_id = %id,
        "Deleting query history item"
    );

    match sqlx::query("DELETE FROM query_history WHERE id = ?")
        .bind(&id)
        .execute(&state.db_pool)
        .await
    {
        Ok(result) => {
            if result.rows_affected() > 0 {
                tracing::info!(
                    module = "handlers::history",
                    event = "delete_query_history_item_success",
                    history_id = %id,
                    "Query history item deleted"
                );
                (
                    StatusCode::OK,
                    Json(ApiResponse::ok("History item deleted".to_string())),
                )
            } else {
                tracing::warn!(
                    module = "handlers::history",
                    event = "delete_query_history_item_not_found",
                    history_id = %id,
                    "Query history item not found"
                );
                (
                    StatusCode::NOT_FOUND,
                    Json(ApiResponse::err("History item not found")),
                )
            }
        }
        Err(e) => {
            tracing::error!(
                module = "handlers::history",
                event = "delete_query_history_item_failed",
                history_id = %id,
                error = %e,
                "Failed to delete query history item"
            );
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::err("Failed to delete query history item")),
            )
        }
    }
}
