use crate::handlers::query::get_or_create_pool;
use crate::state::{AppState, DynPool};
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};
use serde_json::Value;
use sql_admin_shared::{
    ApiResponse, DeleteRowRequest, EditRowRequest, InsertRowRequest, QueryResult,
};
use std::sync::Arc;
use std::time::Instant;

pub async fn edit_row(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(req): Json<EditRowRequest>,
) -> (StatusCode, Json<ApiResponse<QueryResult>>) {
    tracing::info!(
        module = "handlers::data_edit",
        event = "edit_row_start",
        connection_id = %id,
        table = %req.table_name,
        column = %req.column,
        pk_column = %req.primary_key_column,
        "Editing table row"
    );

    let conn_result = crate::db::queries::get_connection(&state.db_pool, id.clone()).await;
    match conn_result {
        Ok(conn) => {
            let pool = match get_or_create_pool(&state, &conn).await {
                Ok(p) => p,
                Err(e) => {
                    tracing::error!(
                        module = "handlers::data_edit",
                        event = "pool_creation_failed",
                        connection_id = %id,
                        error = %e,
                        "Failed to create connection pool for edit row"
                    );
                    return (StatusCode::BAD_REQUEST, Json(ApiResponse::err(e)));
                }
            };

            let safe_table = req.table_name.replace('\"', "\"\"");
            let safe_column = req.column.replace('\"', "\"\"");
            let safe_pk = req.primary_key_column.replace('\"', "\"\"");

            let pk_value_str = value_to_sql_literal(&req.primary_key_value);

            let query = match &req.new_value {
                Some(new_val) => {
                    let new_val_str = value_to_sql_literal(new_val);
                    format!(
                        "UPDATE \"{}\" SET \"{}\" = {} WHERE \"{}\" = {}",
                        safe_table, safe_column, new_val_str, safe_pk, pk_value_str
                    )
                }
                None => {
                    format!(
                        "UPDATE \"{}\" SET \"{}\" = NULL WHERE \"{}\" = {}",
                        safe_table, safe_column, safe_pk, pk_value_str
                    )
                }
            };

            let start = Instant::now();
            let result = match pool {
                DynPool::Postgres(p) => {
                    crate::handlers::query::execute_postgres_query(&p, &query).await
                }
                DynPool::MySql(p) => crate::handlers::query::execute_mysql_query(&p, &query).await,
                DynPool::Sqlite(p) => {
                    crate::handlers::query::execute_sqlite_query(&p, &query).await
                }
            };
            let elapsed = start.elapsed().as_millis() as u64;

            match result {
                Ok(mut qr) => {
                    qr.execution_time_ms = Some(elapsed);
                    tracing::info!(
                        module = "handlers::data_edit",
                        event = "edit_row_success",
                        connection_id = %id,
                        table = %req.table_name,
                        elapsed_ms = elapsed,
                        "Row edited successfully"
                    );
                    (StatusCode::OK, Json(ApiResponse::ok(qr)))
                }
                Err(e) => {
                    tracing::error!(
                        module = "handlers::data_edit",
                        event = "edit_row_failed",
                        connection_id = %id,
                        table = %req.table_name,
                        error = %e,
                        "Failed to edit row"
                    );
                    (StatusCode::BAD_REQUEST, Json(ApiResponse::err(e)))
                }
            }
        }
        Err(_) => {
            tracing::warn!(
                module = "handlers::data_edit",
                event = "connection_not_found",
                connection_id = %id,
                "Connection not found for edit row"
            );
            (
                StatusCode::NOT_FOUND,
                Json(ApiResponse::err("Connection not found")),
            )
        }
    }
}

pub async fn delete_row(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(req): Json<DeleteRowRequest>,
) -> (StatusCode, Json<ApiResponse<QueryResult>>) {
    tracing::info!(
        module = "handlers::data_edit",
        event = "delete_row_start",
        connection_id = %id,
        table = %req.table_name,
        pk_column = %req.primary_key_column,
        "Deleting table row"
    );

    let conn_result = crate::db::queries::get_connection(&state.db_pool, id.clone()).await;
    match conn_result {
        Ok(conn) => {
            let pool = match get_or_create_pool(&state, &conn).await {
                Ok(p) => p,
                Err(e) => {
                    tracing::error!(
                        module = "handlers::data_edit",
                        event = "pool_creation_failed",
                        connection_id = %id,
                        error = %e,
                        "Failed to create connection pool for delete row"
                    );
                    return (StatusCode::BAD_REQUEST, Json(ApiResponse::err(e)));
                }
            };

            let safe_table = req.table_name.replace('\"', "\"\"");
            let safe_pk = req.primary_key_column.replace('\"', "\"\"");
            let pk_value_str = value_to_sql_literal(&req.primary_key_value);

            let query = format!(
                "DELETE FROM \"{}\" WHERE \"{}\" = {}",
                safe_table, safe_pk, pk_value_str
            );

            let start = Instant::now();
            let result = match pool {
                DynPool::Postgres(p) => {
                    crate::handlers::query::execute_postgres_query(&p, &query).await
                }
                DynPool::MySql(p) => crate::handlers::query::execute_mysql_query(&p, &query).await,
                DynPool::Sqlite(p) => {
                    crate::handlers::query::execute_sqlite_query(&p, &query).await
                }
            };
            let elapsed = start.elapsed().as_millis() as u64;

            match result {
                Ok(mut qr) => {
                    qr.execution_time_ms = Some(elapsed);
                    tracing::info!(
                        module = "handlers::data_edit",
                        event = "delete_row_success",
                        connection_id = %id,
                        table = %req.table_name,
                        elapsed_ms = elapsed,
                        "Row deleted successfully"
                    );
                    (StatusCode::OK, Json(ApiResponse::ok(qr)))
                }
                Err(e) => {
                    tracing::error!(
                        module = "handlers::data_edit",
                        event = "delete_row_failed",
                        connection_id = %id,
                        table = %req.table_name,
                        error = %e,
                        "Failed to delete row"
                    );
                    (StatusCode::BAD_REQUEST, Json(ApiResponse::err(e)))
                }
            }
        }
        Err(_) => {
            tracing::warn!(
                module = "handlers::data_edit",
                event = "connection_not_found",
                connection_id = %id,
                "Connection not found for delete row"
            );
            (
                StatusCode::NOT_FOUND,
                Json(ApiResponse::err("Connection not found")),
            )
        }
    }
}

pub async fn insert_row(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(req): Json<InsertRowRequest>,
) -> (StatusCode, Json<ApiResponse<QueryResult>>) {
    tracing::info!(
        module = "handlers::data_edit",
        event = "insert_row_start",
        connection_id = %id,
        table = %req.table_name,
        column_count = req.columns.len(),
        "Inserting table row"
    );

    let conn_result = crate::db::queries::get_connection(&state.db_pool, id.clone()).await;
    match conn_result {
        Ok(conn) => {
            let pool = match get_or_create_pool(&state, &conn).await {
                Ok(p) => p,
                Err(e) => {
                    tracing::error!(
                        module = "handlers::data_edit",
                        event = "pool_creation_failed",
                        connection_id = %id,
                        error = %e,
                        "Failed to create connection pool for insert row"
                    );
                    return (StatusCode::BAD_REQUEST, Json(ApiResponse::err(e)));
                }
            };

            let safe_table = req.table_name.replace('\"', "\"\"");
            let safe_columns: Vec<String> = req
                .columns
                .iter()
                .map(|c| format!("\"{}\"", c.replace('\"', "\"\"")))
                .collect();
            let values: Vec<String> = req.values.iter().map(value_to_sql_literal).collect();

            let query = format!(
                "INSERT INTO \"{}\" ({}) VALUES ({})",
                safe_table,
                safe_columns.join(", "),
                values.join(", ")
            );

            let start = Instant::now();
            let result = match pool {
                DynPool::Postgres(p) => {
                    crate::handlers::query::execute_postgres_query(&p, &query).await
                }
                DynPool::MySql(p) => crate::handlers::query::execute_mysql_query(&p, &query).await,
                DynPool::Sqlite(p) => {
                    crate::handlers::query::execute_sqlite_query(&p, &query).await
                }
            };
            let elapsed = start.elapsed().as_millis() as u64;

            match result {
                Ok(mut qr) => {
                    qr.execution_time_ms = Some(elapsed);
                    tracing::info!(
                        module = "handlers::data_edit",
                        event = "insert_row_success",
                        connection_id = %id,
                        table = %req.table_name,
                        elapsed_ms = elapsed,
                        "Row inserted successfully"
                    );
                    (StatusCode::OK, Json(ApiResponse::ok(qr)))
                }
                Err(e) => {
                    tracing::error!(
                        module = "handlers::data_edit",
                        event = "insert_row_failed",
                        connection_id = %id,
                        table = %req.table_name,
                        error = %e,
                        "Failed to insert row"
                    );
                    (StatusCode::BAD_REQUEST, Json(ApiResponse::err(e)))
                }
            }
        }
        Err(_) => {
            tracing::warn!(
                module = "handlers::data_edit",
                event = "connection_not_found",
                connection_id = %id,
                "Connection not found for insert row"
            );
            (
                StatusCode::NOT_FOUND,
                Json(ApiResponse::err("Connection not found")),
            )
        }
    }
}

fn value_to_sql_literal(val: &Value) -> String {
    match val {
        Value::Null => "NULL".to_string(),
        Value::Bool(b) => {
            if *b {
                "1".to_string()
            } else {
                "0".to_string()
            }
        }
        Value::Number(n) => n.to_string(),
        Value::String(s) => {
            let escaped = s.replace('\'', "''");
            format!("'{}'", escaped)
        }
        Value::Array(arr) => {
            let s = serde_json::to_string(arr).unwrap_or_default();
            let escaped = s.replace('\'', "''");
            format!("'{}'", escaped)
        }
        Value::Object(obj) => {
            let s = serde_json::to_string(obj).unwrap_or_default();
            let escaped = s.replace('\'', "''");
            format!("'{}'", escaped)
        }
    }
}
