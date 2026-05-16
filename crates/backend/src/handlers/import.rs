
use crate::db::queries;
use crate::handlers::query::{execute_mysql_query, execute_postgres_query, execute_sqlite_query, get_or_create_pool};
use crate::state::{AppState, DynPool};
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};
use sql_admin_shared::{ApiResponse, AppError, ImportResult, ImportSqlRequest};
use std::sync::Arc;
use std::time::Instant;

fn split_sql_statements(sql: &str) -> Vec<String> {
    let mut statements = Vec::new();
    let mut current = String::new();
    let mut in_single_quote = false;
    let mut in_double_quote = false;
    let mut chars = sql.chars().peekable();

    while let Some(ch) = chars.next() {
        if in_single_quote {
            current.push(ch);
            if ch == '\'' {
                if chars.peek() == Some(&'\'') {
                    current.push(chars.next().unwrap());
                } else {
                    in_single_quote = false;
                }
            }
        } else if in_double_quote {
            current.push(ch);
            if ch == '"' {
                if chars.peek() == Some(&'"') {
                    current.push(chars.next().unwrap());
                } else {
                    in_double_quote = false;
                }
            }
        } else if ch == '-' && chars.peek() == Some(&'-') {
            current.push(ch);
            current.push(chars.next().unwrap());
            while let Some(c) = chars.next() {
                current.push(c);
                if c == '\n' {
                    break;
                }
            }
        } else if ch == '/' && chars.peek() == Some(&'*') {
            current.push(ch);
            current.push(chars.next().unwrap());
            while let Some(c) = chars.next() {
                current.push(c);
                if c == '*' && chars.peek() == Some(&'/') {
                    current.push(chars.next().unwrap());
                    break;
                }
            }
        } else if ch == '\'' {
            current.push(ch);
            in_single_quote = true;
        } else if ch == '"' {
            current.push(ch);
            in_double_quote = true;
        } else if ch == ';' {
            let trimmed = current.trim().to_string();
            if !trimmed.is_empty() {
                statements.push(trimmed);
            }
            current.clear();
        } else {
            current.push(ch);
        }
    }

    let trimmed = current.trim().to_string();
    if !trimmed.is_empty() {
        statements.push(trimmed);
    }

    statements
}

pub async fn import_sql(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(req): Json<ImportSqlRequest>,
) -> (StatusCode, Json<ApiResponse<ImportResult>>) {
    tracing::info!(
        module = "handlers::import",
        event = "import_sql_start",
        connection_id = %id,
        content_length = req.sql_content.len(),
        "Starting SQL import"
    );

    let conn_result = queries::get_connection(&state.db_pool, id.clone()).await;

    let conn = match conn_result {
        Ok(c) => c,
        Err(AppError::ConnectionNotFound) => {
            tracing::warn!(
                module = "handlers::import",
                event = "connection_not_found",
                connection_id = %id,
                "Connection not found for SQL import"
            );
            return (
                StatusCode::NOT_FOUND,
                Json(ApiResponse::err("Connection not found")),
            );
        }
        Err(e) => {
            tracing::error!(
                module = "handlers::import",
                event = "get_connection_failed",
                connection_id = %id,
                error = %e,
                "Failed to get connection for SQL import"
            );
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::err("Failed to get connection")),
            );
        }
    };

    let pool = match get_or_create_pool(&state, &conn).await {
        Ok(p) => p,
        Err(e) => {
            tracing::error!(
                module = "handlers::import",
                event = "pool_creation_failed",
                connection_id = %id,
                error = %e,
                "Failed to create connection pool for SQL import"
            );
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::err(format!("Connection failed: {}", e))),
            );
        }
    };

    let statements = split_sql_statements(&req.sql_content);
    let total = statements.len();
    tracing::info!(
        module = "handlers::import",
        event = "import_sql_parsed",
        connection_id = %id,
        total_statements = total,
        "SQL statements parsed"
    );

    let mut errors = Vec::new();
    let start = Instant::now();
    let mut executed = 0u32;

    for stmt in &statements {
        let result = match &pool {
            DynPool::Postgres(p) => execute_postgres_query(p, stmt).await,
            DynPool::MySql(p) => execute_mysql_query(p, stmt).await,
            DynPool::Sqlite(p) => execute_sqlite_query(p, stmt).await,
        };

        match result {
            Ok(_) => executed += 1,
            Err(e) => {
                let preview: String = if stmt.len() > 80 {
                    format!("{}...", &stmt[..80])
                } else {
                    stmt.clone()
                };
                tracing::error!(
                    module = "handlers::import",
                    event = "import_statement_failed",
                    connection_id = %id,
                    statement_index = executed + 1,
                    statement_preview = %preview,
                    error = %e,
                    "SQL statement execution failed during import"
                );
                errors.push(format!("[Statement {}] {}: {}", executed + 1, preview, e));
            }
        }
    }

    let elapsed = start.elapsed().as_millis() as u64;

    let import_result = ImportResult {
        statements_executed: executed,
        errors,
        execution_time_ms: Some(elapsed),
    };

    if import_result.statements_executed == 0 && !import_result.errors.is_empty() {
        tracing::error!(
            module = "handlers::import",
            event = "import_sql_all_failed",
            connection_id = %id,
            total_statements = total,
            elapsed_ms = elapsed,
            "All SQL statements failed during import"
        );
        (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::err(format!(
                "All {} statements failed",
                total
            ))),
        )
    } else {
        tracing::info!(
            module = "handlers::import",
            event = "import_sql_completed",
            connection_id = %id,
            total_statements = total,
            executed = executed,
            failed = total - executed as usize,
            elapsed_ms = elapsed,
            "SQL import completed"
        );
        (StatusCode::OK, Json(ApiResponse::ok(import_result)))
    }
}
