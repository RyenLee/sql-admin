use crate::db::queries;
use crate::state::AppState;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use sql_admin_shared::{
    ApiResponse, AppError, Connection, CreateConnectionRequest, UpdateConnectionRequest,
};
use std::sync::Arc;

pub async fn create_connection(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateConnectionRequest>,
) -> (StatusCode, Json<ApiResponse<Connection>>) {
    tracing::info!(
        module = "handlers::connections",
        event = "create_connection_start",
        name = %req.name,
        database_type = %req.database_type,
        host = %req.host,
        port = req.port,
        database = %req.database,
        "Creating new database connection"
    );

    let result = queries::create_connection(
        &state.db_pool,
        &req.name,
        req.database_type,
        &req.host,
        req.port,
        &req.database,
        &req.username,
        req.password.as_deref(),
    )
    .await;

    match result {
        Ok(conn) => {
            tracing::info!(
                module = "handlers::connections",
                event = "create_connection_success",
                connection_id = %conn.id,
                name = %conn.name,
                "Database connection created successfully"
            );
            (StatusCode::CREATED, Json(ApiResponse::ok(conn)))
        }
        Err(e) => {
            tracing::error!(
                module = "handlers::connections",
                event = "create_connection_failed",
                name = %req.name,
                error = %e,
                error_type = std::any::type_name_of_val(&e),
                "Failed to create connection"
            );
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::err("Failed to create connection")),
            )
        }
    }
}

pub async fn list_connections(
    State(state): State<Arc<AppState>>,
) -> Json<ApiResponse<Vec<Connection>>> {
    tracing::info!(
        module = "handlers::connections",
        event = "list_connections_start",
        "Fetching all database connections"
    );

    let result = queries::list_connections(&state.db_pool).await;

    match result {
        Ok(conns) => {
            tracing::info!(
                module = "handlers::connections",
                event = "list_connections_success",
                count = conns.len(),
                "Fetched connections successfully"
            );
            Json(ApiResponse::ok(conns))
        }
        Err(e) => {
            tracing::error!(
                module = "handlers::connections",
                event = "list_connections_failed",
                error = %e,
                "Failed to fetch connections"
            );
            Json(ApiResponse::err("Failed to fetch connections"))
        }
    }
}

pub async fn get_connection(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> (StatusCode, Json<ApiResponse<Connection>>) {
    tracing::info!(
        module = "handlers::connections",
        event = "get_connection_start",
        connection_id = %id,
        "Fetching connection by ID"
    );

    let result = queries::get_connection(&state.db_pool, id.clone()).await;

    match result {
        Ok(conn) => {
            tracing::info!(
                module = "handlers::connections",
                event = "get_connection_success",
                connection_id = %id,
                name = %conn.name,
                "Connection fetched successfully"
            );
            (StatusCode::OK, Json(ApiResponse::ok(conn)))
        }
        Err(AppError::ConnectionNotFound) => {
            tracing::warn!(
                module = "handlers::connections",
                event = "get_connection_not_found",
                connection_id = %id,
                "Connection not found"
            );
            (StatusCode::NOT_FOUND, Json(ApiResponse::err("Connection not found")))
        }
        Err(e) => {
            tracing::error!(
                module = "handlers::connections",
                event = "get_connection_failed",
                connection_id = %id,
                error = %e,
                "Failed to get connection"
            );
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::err("Failed to get connection")))
        }
    }
}

pub async fn update_connection(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(req): Json<UpdateConnectionRequest>,
) -> (StatusCode, Json<ApiResponse<Connection>>) {
    tracing::info!(
        module = "handlers::connections",
        event = "update_connection_start",
        connection_id = %id,
        "Updating connection"
    );

    let result = queries::update_connection(
        &state.db_pool,
        id.clone(),
        req.name.as_deref(),
        req.host.as_deref(),
        req.port.map(|p| p as i32),
        req.database.as_deref(),
        req.username.as_deref(),
        req.password.as_deref(),
    )
    .await;

    match result {
        Ok(conn) => {
            let mut pools = state.connection_pools.write().await;
            pools.remove(&id);
            tracing::info!(
                module = "handlers::connections",
                event = "update_connection_success",
                connection_id = %id,
                name = %conn.name,
                "Connection updated successfully, pool cache invalidated"
            );
            (StatusCode::OK, Json(ApiResponse::ok(conn)))
        }
        Err(AppError::ConnectionNotFound) => {
            tracing::warn!(
                module = "handlers::connections",
                event = "update_connection_not_found",
                connection_id = %id,
                "Connection not found for update"
            );
            (StatusCode::NOT_FOUND, Json(ApiResponse::err("Connection not found")))
        }
        Err(e) => {
            tracing::error!(
                module = "handlers::connections",
                event = "update_connection_failed",
                connection_id = %id,
                error = %e,
                "Failed to update connection"
            );
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::err("Failed to update connection")))
        }
    }
}

pub async fn delete_connection(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> (StatusCode, Json<ApiResponse<String>>) {
    tracing::info!(
        module = "handlers::connections",
        event = "delete_connection_start",
        connection_id = %id,
        "Deleting connection"
    );

    match queries::delete_connection(&state.db_pool, id.clone()).await {
        Ok(true) => {
            let mut pools = state.connection_pools.write().await;
            pools.remove(&id);
            tracing::info!(
                module = "handlers::connections",
                event = "delete_connection_success",
                connection_id = %id,
                "Connection deleted successfully, pool cache invalidated"
            );
            (StatusCode::OK, Json(ApiResponse::ok("Connection deleted".to_string())))
        }
        Ok(false) => {
            tracing::warn!(
                module = "handlers::connections",
                event = "delete_connection_not_found",
                connection_id = %id,
                "Connection not found for deletion"
            );
            (StatusCode::NOT_FOUND, Json(ApiResponse::err("Connection not found")))
        }
        Err(e) => {
            tracing::error!(
                module = "handlers::connections",
                event = "delete_connection_failed",
                connection_id = %id,
                error = %e,
                "Failed to delete connection"
            );
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::err("Failed to delete connection")))
        }
    }
}

pub async fn test_connection(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Json<ApiResponse<String>> {
    tracing::info!(
        module = "handlers::connections",
        event = "test_connection_start",
        connection_id = %id,
        "Testing existing connection"
    );

    let conn_result = queries::get_connection(&state.db_pool, id).await;

    match conn_result {
        Ok(conn) => {
            let result = test_db_connection(
                conn.database_type,
                &conn.host,
                conn.port,
                &conn.database,
                &conn.username,
                conn.password.as_deref(),
            )
            .await;

            match result {
                Ok(_) => {
                    tracing::info!(
                        module = "handlers::connections",
                        event = "test_connection_success",
                        connection_id = %conn.id,
                        "Connection test successful"
                    );
                    Json(ApiResponse::ok("Connection successful!".to_string()))
                }
                Err(e) => {
                    tracing::error!(
                        module = "handlers::connections",
                        event = "test_connection_failed",
                        connection_id = %conn.id,
                        error = %e,
                        "Connection test failed"
                    );
                    Json(ApiResponse::err(format!("Connection failed: {}", e)))
                }
            }
        }
        Err(e) => {
            tracing::error!(
                module = "handlers::connections",
                event = "test_connection_get_failed",
                error = %e,
                "Failed to get connection for testing"
            );
            Json(ApiResponse::err("Connection not found"))
        }
    }
}

pub async fn test_connection_request(
    Json(req): Json<CreateConnectionRequest>,
) -> Json<ApiResponse<String>> {
    tracing::info!(
        module = "handlers::connections",
        event = "test_connection_request_start",
        database_type = %req.database_type,
        host = %req.host,
        port = req.port,
        database = %req.database,
        "Testing connection with provided parameters"
    );

    let result = test_db_connection(
        req.database_type,
        &req.host,
        req.port,
        &req.database,
        &req.username,
        req.password.as_deref(),
    )
    .await;

    match result {
        Ok(_) => {
            tracing::info!(
                module = "handlers::connections",
                event = "test_connection_request_success",
                host = %req.host,
                "Connection test with parameters successful"
            );
            Json(ApiResponse::ok("Connection successful!".to_string()))
        }
        Err(e) => {
            tracing::error!(
                module = "handlers::connections",
                event = "test_connection_request_failed",
                host = %req.host,
                error = %e,
                "Connection test with parameters failed"
            );
            Json(ApiResponse::err(format!("Connection failed: {}", e)))
        }
    }
}

async fn test_db_connection(
    database_type: sql_admin_shared::DatabaseType,
    host: &str,
    port: u16,
    database: &str,
    username: &str,
    password: Option<&str>,
) -> Result<(), String> {
    match database_type {
        sql_admin_shared::DatabaseType::Postgres => {
            let conn_str = format!(
                "postgres://{}:{}@{}:{}/{}",
                username,
                password.unwrap_or_default(),
                host,
                port,
                database
            );
            test_postgres_connection(&conn_str).await
        }
        sql_admin_shared::DatabaseType::Mysql => {
            let conn_str = format!(
                "mysql://{}:{}@{}:{}/{}",
                username,
                password.unwrap_or_default(),
                host,
                port,
                database
            );
            test_mysql_connection(&conn_str).await
        }
        sql_admin_shared::DatabaseType::Sqlite => test_sqlite_connection(database).await,
    }
}

async fn test_postgres_connection(conn_str: &str) -> Result<(), String> {
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .connect(conn_str)
        .await
        .map(|_| ())
        .map_err(|e| e.to_string())
}

async fn test_mysql_connection(conn_str: &str) -> Result<(), String> {
    sqlx::mysql::MySqlPoolOptions::new()
        .max_connections(1)
        .connect(conn_str)
        .await
        .map(|_| ())
        .map_err(|e| e.to_string())
}

async fn test_sqlite_connection(path: &str) -> Result<(), String> {
    let conn_str = format!("sqlite://{}?mode=rwc", path);
    sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(1)
        .connect(&conn_str)
        .await
        .map(|_| ())
        .map_err(|e| e.to_string())
}
