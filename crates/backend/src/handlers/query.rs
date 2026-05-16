use crate::db::queries;
use crate::state::{AppState, DynPool};
use axum::{
    extract::{Json, Path, Query, State},
    http::StatusCode,
};
use serde::Deserialize;
use serde_json::Value;
use sql_admin_shared::{ApiResponse, AppError, ExecuteQueryRequest, QueryResult};
use sqlx::{Column, Row, mysql::MySqlRow, postgres::PgRow, sqlite::SqliteRow};
use std::sync::Arc;
use std::time::Instant;

const BLOCKED_KEYWORDS: &[&str] = &[
    "DROP DATABASE",
    "SHUTDOWN",
    "DROP TABLE mysql",
    "DROP TABLE pg_",
    "GRANT ALL",
    "ALTER USER",
];

fn validate_query(query: &str) -> Result<(), String> {
    let upper = query.to_uppercase();
    for keyword in BLOCKED_KEYWORDS {
        if upper.contains(keyword) {
            return Err(format!("Dangerous SQL operation blocked: {}", keyword));
        }
    }
    Ok(())
}

pub async fn get_or_create_pool(
    state: &Arc<AppState>,
    conn: &sql_admin_shared::Connection,
) -> Result<DynPool, String> {
    {
        let pools = state.connection_pools.read().await;
        if let Some(pool) = pools.get(&conn.id) {
            return Ok(pool.clone());
        }
    }

    tracing::info!(
        module = "handlers::query",
        event = "create_pool",
        connection_id = %conn.id,
        database_type = %conn.database_type,
        "Creating new connection pool"
    );

    let pool = match conn.database_type {
        sql_admin_shared::DatabaseType::Postgres => {
            let conn_str = format!(
                "postgres://{}:{}@{}:{}/{}",
                conn.username,
                conn.password.as_deref().unwrap_or_default(),
                conn.host,
                conn.port,
                conn.database
            );
            let p = sqlx::postgres::PgPoolOptions::new()
                .max_connections(5)
                .connect(&conn_str)
                .await
                .map_err(|e| e.to_string())?;
            DynPool::Postgres(p)
        }
        sql_admin_shared::DatabaseType::Mysql => {
            let conn_str = format!(
                "mysql://{}:{}@{}:{}/{}",
                conn.username,
                conn.password.as_deref().unwrap_or_default(),
                conn.host,
                conn.port,
                conn.database
            );
            let p = sqlx::mysql::MySqlPoolOptions::new()
                .max_connections(5)
                .connect(&conn_str)
                .await
                .map_err(|e| e.to_string())?;
            DynPool::MySql(p)
        }
        sql_admin_shared::DatabaseType::Sqlite => {
            let conn_str = format!("sqlite://{}?mode=rwc", conn.database);
            let p = sqlx::sqlite::SqlitePoolOptions::new()
                .max_connections(5)
                .connect(&conn_str)
                .await
                .map_err(|e| e.to_string())?;
            DynPool::Sqlite(p)
        }
    };

    {
        let mut pools = state.connection_pools.write().await;
        pools.insert(conn.id.clone(), pool.clone());
    }

    tracing::info!(
        module = "handlers::query",
        event = "pool_created",
        connection_id = %conn.id,
        "Connection pool created and cached"
    );

    Ok(pool)
}

pub async fn execute_query(
    State(state): State<Arc<AppState>>,
    Json(req): Json<ExecuteQueryRequest>,
) -> (StatusCode, Json<ApiResponse<QueryResult>>) {
    tracing::info!(
        module = "handlers::query",
        event = "execute_query_start",
        connection_id = %req.connection_id,
        query_length = req.query.len(),
        "Executing SQL query"
    );

    if let Err(e) = validate_query(&req.query) {
        tracing::warn!(
            module = "handlers::query",
            event = "query_blocked",
            connection_id = %req.connection_id,
            reason = %e,
            "Query blocked by validation"
        );
        return (StatusCode::BAD_REQUEST, Json(ApiResponse::err(e)));
    }

    let conn_result = queries::get_connection(&state.db_pool, req.connection_id.clone()).await;

    match conn_result {
        Ok(conn) => {
            let pool = match get_or_create_pool(&state, &conn).await {
                Ok(p) => p,
                Err(e) => {
                    tracing::error!(
                        module = "handlers::query",
                        event = "pool_creation_failed",
                        connection_id = %req.connection_id,
                        error = %e,
                        "Failed to create connection pool"
                    );
                    return (
                        StatusCode::BAD_REQUEST,
                        Json(ApiResponse::err(format!("Connection failed: {}", e))),
                    );
                }
            };

            let start = Instant::now();
            let result = match pool {
                DynPool::Postgres(p) => execute_postgres_query(&p, &req.query).await,
                DynPool::MySql(p) => execute_mysql_query(&p, &req.query).await,
                DynPool::Sqlite(p) => execute_sqlite_query(&p, &req.query).await,
            };
            let elapsed = start.elapsed().as_millis() as u64;

            match result {
                Ok(mut query_result) => {
                    query_result.execution_time_ms = Some(elapsed);
                    tracing::info!(
                        module = "handlers::query",
                        event = "execute_query_success",
                        connection_id = %req.connection_id,
                        elapsed_ms = elapsed,
                        row_count = query_result.rows.len(),
                        "Query executed successfully"
                    );
                    (StatusCode::OK, Json(ApiResponse::ok(query_result)))
                }
                Err(e) => {
                    tracing::error!(
                        module = "handlers::query",
                        event = "execute_query_failed",
                        connection_id = %req.connection_id,
                        elapsed_ms = elapsed,
                        error = %e,
                        "Query execution failed"
                    );
                    (
                        StatusCode::BAD_REQUEST,
                        Json(ApiResponse::err(e.to_string())),
                    )
                }
            }
        }
        Err(AppError::ConnectionNotFound) => {
            tracing::warn!(
                module = "handlers::query",
                event = "connection_not_found",
                connection_id = %req.connection_id,
                "Connection not found for query execution"
            );
            (
                StatusCode::NOT_FOUND,
                Json(ApiResponse::err("Connection not found")),
            )
        }
        Err(e) => {
            tracing::error!(
                module = "handlers::query",
                event = "get_connection_failed",
                connection_id = %req.connection_id,
                error = %e,
                "Failed to get connection for query"
            );
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::err("Failed to execute query")),
            )
        }
    }
}

#[derive(Deserialize)]
pub struct TableDataParams {
    #[serde(default = "default_limit")]
    pub limit: i64,
    #[serde(default = "default_offset")]
    pub offset: i64,
}

fn default_limit() -> i64 {
    100
}
fn default_offset() -> i64 {
    0
}

pub async fn get_table_data(
    State(state): State<Arc<AppState>>,
    Path((id, table)): Path<(String, String)>,
    Query(params): Query<TableDataParams>,
) -> (StatusCode, Json<ApiResponse<QueryResult>>) {
    tracing::info!(
        module = "handlers::query",
        event = "get_table_data_start",
        connection_id = %id,
        table = %table,
        limit = params.limit,
        offset = params.offset,
        "Fetching table data"
    );

    let conn_result = queries::get_connection(&state.db_pool, id.clone()).await;

    match conn_result {
        Ok(conn) => {
            let pool = match get_or_create_pool(&state, &conn).await {
                Ok(p) => p,
                Err(e) => {
                    tracing::error!(
                        module = "handlers::query",
                        event = "pool_creation_failed",
                        connection_id = %id,
                        error = %e,
                        "Failed to create connection pool for table data"
                    );
                    return (
                        StatusCode::BAD_REQUEST,
                        Json(ApiResponse::err(format!("Connection failed: {}", e))),
                    );
                }
            };

            let safe_table = table.replace('\"', "\"\"");
            let query = format!(
                "SELECT * FROM \"{}\" LIMIT {} OFFSET {}",
                safe_table, params.limit, params.offset
            );

            let start = Instant::now();
            let result = match pool {
                DynPool::Postgres(p) => execute_postgres_query(&p, &query).await,
                DynPool::MySql(p) => execute_mysql_query(&p, &query).await,
                DynPool::Sqlite(p) => execute_sqlite_query(&p, &query).await,
            };
            let elapsed = start.elapsed().as_millis() as u64;

            match result {
                Ok(mut query_result) => {
                    query_result.execution_time_ms = Some(elapsed);
                    tracing::info!(
                        module = "handlers::query",
                        event = "get_table_data_success",
                        connection_id = %id,
                        table = %table,
                        elapsed_ms = elapsed,
                        row_count = query_result.rows.len(),
                        "Table data fetched successfully"
                    );
                    (StatusCode::OK, Json(ApiResponse::ok(query_result)))
                }
                Err(e) => {
                    tracing::error!(
                        module = "handlers::query",
                        event = "get_table_data_failed",
                        connection_id = %id,
                        table = %table,
                        error = %e,
                        "Failed to fetch table data"
                    );
                    (
                        StatusCode::BAD_REQUEST,
                        Json(ApiResponse::err(e.to_string())),
                    )
                }
            }
        }
        Err(_) => {
            tracing::warn!(
                module = "handlers::query",
                event = "connection_not_found",
                connection_id = %id,
                "Connection not found for table data"
            );
            (
                StatusCode::NOT_FOUND,
                Json(ApiResponse::err("Connection not found")),
            )
        }
    }
}

pub async fn execute_postgres_query(
    pool: &sqlx::postgres::PgPool,
    query: &str,
) -> Result<QueryResult, String> {
    let result = sqlx::query(query)
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;

    if result.is_empty() {
        let rows_affected = sqlx::query(query)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?
            .rows_affected();

        Ok(QueryResult {
            columns: vec!["affected_rows".to_string()],
            rows: vec![vec![Value::Number(rows_affected.into())]],
            rows_affected: Some(rows_affected),
            execution_time_ms: None,
        })
    } else {
        let columns: Vec<String> = result[0]
            .columns()
            .iter()
            .map(|c| c.name().to_string())
            .collect();

        let rows: Vec<Vec<Value>> = result
            .into_iter()
            .map(|row| {
                columns
                    .iter()
                    .map(|col| pg_row_to_value(&row, col))
                    .collect()
            })
            .collect();

        Ok(QueryResult {
            columns,
            rows,
            rows_affected: None,
            execution_time_ms: None,
        })
    }
}

pub async fn execute_mysql_query(
    pool: &sqlx::mysql::MySqlPool,
    query: &str,
) -> Result<QueryResult, String> {
    let result = sqlx::query(query)
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;

    if result.is_empty() {
        let rows_affected = sqlx::query(query)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?
            .rows_affected();

        Ok(QueryResult {
            columns: vec!["affected_rows".to_string()],
            rows: vec![vec![Value::Number(rows_affected.into())]],
            rows_affected: Some(rows_affected),
            execution_time_ms: None,
        })
    } else {
        let columns: Vec<String> = result[0]
            .columns()
            .iter()
            .map(|c| c.name().to_string())
            .collect();

        let rows: Vec<Vec<Value>> = result
            .into_iter()
            .map(|row| {
                columns
                    .iter()
                    .map(|col| mysql_row_to_value(&row, col))
                    .collect()
            })
            .collect();

        Ok(QueryResult {
            columns,
            rows,
            rows_affected: None,
            execution_time_ms: None,
        })
    }
}

pub async fn execute_sqlite_query(
    pool: &sqlx::sqlite::SqlitePool,
    query: &str,
) -> Result<QueryResult, String> {
    let result = sqlx::query(query)
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;

    if result.is_empty() {
        let rows_affected = sqlx::query(query)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?
            .rows_affected();

        Ok(QueryResult {
            columns: vec!["affected_rows".to_string()],
            rows: vec![vec![Value::Number(rows_affected.into())]],
            rows_affected: Some(rows_affected),
            execution_time_ms: None,
        })
    } else {
        let columns: Vec<String> = result[0]
            .columns()
            .iter()
            .map(|c| c.name().to_string())
            .collect();

        let rows: Vec<Vec<Value>> = result
            .into_iter()
            .map(|row| {
                columns
                    .iter()
                    .map(|col| sqlite_row_to_value(&row, col))
                    .collect()
            })
            .collect();

        Ok(QueryResult {
            columns,
            rows,
            rows_affected: None,
            execution_time_ms: None,
        })
    }
}

fn pg_row_to_value(row: &PgRow, col: &str) -> Value {
    match row.try_get::<Option<String>, _>(col) {
        Ok(Some(s)) => Value::String(s),
        Ok(None) => Value::Null,
        Err(_) => match row.try_get::<Option<i64>, _>(col) {
            Ok(Some(n)) => Value::Number(n.into()),
            Ok(None) => Value::Null,
            Err(_) => match row.try_get::<Option<f64>, _>(col) {
                Ok(Some(f)) => Value::Number(serde_json::Number::from_f64(f).unwrap_or(0.into())),
                Ok(None) => Value::Null,
                Err(_) => Value::Null,
            },
        },
    }
}

fn mysql_row_to_value(row: &MySqlRow, col: &str) -> Value {
    match row.try_get::<Option<String>, _>(col) {
        Ok(Some(s)) => Value::String(s),
        Ok(None) => Value::Null,
        Err(_) => match row.try_get::<Option<i64>, _>(col) {
            Ok(Some(n)) => Value::Number(n.into()),
            Ok(None) => Value::Null,
            Err(_) => match row.try_get::<Option<f64>, _>(col) {
                Ok(Some(f)) => Value::Number(serde_json::Number::from_f64(f).unwrap_or(0.into())),
                Ok(None) => Value::Null,
                Err(_) => Value::Null,
            },
        },
    }
}

fn sqlite_row_to_value(row: &SqliteRow, col: &str) -> Value {
    match row.try_get::<Option<String>, _>(col) {
        Ok(Some(s)) => Value::String(s),
        Ok(None) => Value::Null,
        Err(_) => match row.try_get::<Option<i64>, _>(col) {
            Ok(Some(n)) => Value::Number(n.into()),
            Ok(None) => Value::Null,
            Err(_) => match row.try_get::<Option<f64>, _>(col) {
                Ok(Some(f)) => Value::Number(serde_json::Number::from_f64(f).unwrap_or(0.into())),
                Ok(None) => Value::Null,
                Err(_) => Value::Null,
            },
        },
    }
}
