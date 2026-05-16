use crate::db::queries;
use crate::handlers::query::get_or_create_pool;
use crate::state::{AppState, DynPool};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use sql_admin_shared::{ApiResponse, ColumnInfo, IndexInfo, SchemaInfo, TableDef, TableInfo};
use sqlx::Row;
use std::sync::Arc;

pub async fn get_schema(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> (StatusCode, Json<ApiResponse<SchemaInfo>>) {
    tracing::info!(
        module = "handlers::schema",
        event = "get_schema_start",
        connection_id = %id,
        "Fetching database schema"
    );

    let conn_result = queries::get_connection(&state.db_pool, id.clone()).await;

    match conn_result {
        Ok(conn) => {
            let pool = match get_or_create_pool(&state, &conn).await {
                Ok(p) => p,
                Err(e) => {
                    tracing::error!(
                        module = "handlers::schema",
                        event = "pool_creation_failed",
                        connection_id = %id,
                        error = %e,
                        "Failed to create connection pool for schema fetch"
                    );
                    return (
                        StatusCode::BAD_REQUEST,
                        Json(ApiResponse::err(format!("Connection failed: {}", e))),
                    );
                }
            };

            let result = match pool {
                DynPool::Postgres(p) => get_postgres_schema(&p).await,
                DynPool::MySql(p) => get_mysql_schema(&p).await,
                DynPool::Sqlite(p) => get_sqlite_schema(&p).await,
            };

            match result {
                Ok(schema) => {
                    tracing::info!(
                        module = "handlers::schema",
                        event = "get_schema_success",
                        connection_id = %id,
                        table_count = schema.tables.len(),
                        view_count = schema.views.len(),
                        "Schema fetched successfully"
                    );
                    (StatusCode::OK, Json(ApiResponse::ok(schema)))
                }
                Err(e) => {
                    tracing::error!(
                        module = "handlers::schema",
                        event = "get_schema_failed",
                        connection_id = %id,
                        error = %e,
                        "Failed to fetch schema"
                    );
                    (StatusCode::BAD_REQUEST, Json(ApiResponse::err(e)))
                }
            }
        }
        Err(_) => {
            tracing::warn!(
                module = "handlers::schema",
                event = "connection_not_found",
                connection_id = %id,
                "Connection not found for schema fetch"
            );
            (
                StatusCode::NOT_FOUND,
                Json(ApiResponse::err("Connection not found")),
            )
        }
    }
}

async fn get_sqlite_schema(pool: &sqlx::sqlite::SqlitePool) -> Result<SchemaInfo, String> {
    let table_rows = sqlx::query(
        "SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%' ORDER BY name"
    )
    .fetch_all(pool)
    .await
    .map_err(|e| e.to_string())?;

    let mut tables = Vec::new();
    for row in table_rows {
        let table_name: String = row.get("name");

        let col_rows = sqlx::query(&format!(
            "PRAGMA table_info('{}')",
            table_name.replace('\'', "''")
        ))
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;

        let columns: Vec<ColumnInfo> = col_rows
            .iter()
            .map(|r| ColumnInfo {
                name: r.get("name"),
                data_type: r.get("type"),
                not_null: r.get::<i32, _>("notnull") == 1,
                default_value: r.get::<Option<String>, _>("dflt_value"),
                is_primary_key: r.get::<i32, _>("pk") > 0,
            })
            .collect();

        let row_count: Option<i64> = sqlx::query_scalar(&format!(
            "SELECT COUNT(*) FROM \"{}\"",
            table_name.replace('"', "\"\"")
        ))
        .fetch_one(pool)
        .await
        .ok();

        tables.push(TableInfo {
            name: table_name,
            row_count,
            columns,
        });
    }

    let view_rows = sqlx::query("SELECT name FROM sqlite_master WHERE type='view' ORDER BY name")
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;

    let views: Vec<String> = view_rows.iter().map(|r| r.get("name")).collect();

    let idx_rows = sqlx::query(
        "SELECT name, tbl_name, sql FROM sqlite_master WHERE type='index' AND sql IS NOT NULL ORDER BY name"
    )
    .fetch_all(pool)
    .await
    .map_err(|e| e.to_string())?;

    let indexes: Vec<IndexInfo> = idx_rows
        .iter()
        .map(|r| {
            let sql: String = r.get("sql");
            let is_unique = sql.to_uppercase().starts_with("CREATE UNIQUE");
            IndexInfo {
                name: r.get("name"),
                table_name: r.get("tbl_name"),
                columns: vec![],
                is_unique,
            }
        })
        .collect();

    let trigger_rows =
        sqlx::query("SELECT name FROM sqlite_master WHERE type='trigger' ORDER BY name")
            .fetch_all(pool)
            .await
            .map_err(|e| e.to_string())?;

    let triggers: Vec<String> = trigger_rows.iter().map(|r| r.get("name")).collect();

    Ok(SchemaInfo {
        tables,
        views,
        indexes,
        triggers,
        schemas: vec![],
    })
}

async fn get_postgres_schema(pool: &sqlx::postgres::PgPool) -> Result<SchemaInfo, String> {
    let table_rows = sqlx::query(
        "SELECT table_name FROM information_schema.tables WHERE table_schema='public' AND table_type='BASE TABLE' ORDER BY table_name"
    )
    .fetch_all(pool)
    .await
    .map_err(|e| e.to_string())?;

    let mut tables = Vec::new();
    for row in table_rows {
        let table_name: String = row.get("table_name");

        let col_rows = sqlx::query(
            "SELECT column_name, data_type, is_nullable, column_default FROM information_schema.columns WHERE table_schema='public' AND table_name=$1 ORDER BY ordinal_position"
        )
        .bind(&table_name)
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;

        let columns: Vec<ColumnInfo> = col_rows
            .iter()
            .map(|r| {
                let is_nullable: String = r.get("is_nullable");
                ColumnInfo {
                    name: r.get("column_name"),
                    data_type: r.get("data_type"),
                    not_null: is_nullable == "NO",
                    default_value: r.get("column_default"),
                    is_primary_key: false,
                }
            })
            .collect();

        tables.push(TableInfo {
            name: table_name,
            row_count: None,
            columns,
        });
    }

    let view_rows = sqlx::query(
        "SELECT table_name FROM information_schema.views WHERE table_schema='public' ORDER BY table_name"
    )
    .fetch_all(pool)
    .await
    .map_err(|e| e.to_string())?;

    let views: Vec<String> = view_rows.iter().map(|r| r.get("table_name")).collect();

    Ok(SchemaInfo {
        tables,
        views,
        indexes: vec![],
        triggers: vec![],
        schemas: vec![],
    })
}

async fn get_mysql_schema(pool: &sqlx::mysql::MySqlPool) -> Result<SchemaInfo, String> {
    let table_rows = sqlx::query(
        "SELECT TABLE_NAME FROM information_schema.tables WHERE TABLE_SCHEMA=DATABASE() AND TABLE_TYPE='BASE TABLE' ORDER BY TABLE_NAME"
    )
    .fetch_all(pool)
    .await
    .map_err(|e| e.to_string())?;

    let mut tables = Vec::new();
    for row in table_rows {
        let table_name: String = row.get("TABLE_NAME");

        let col_rows = sqlx::query(
            "SELECT CAST(COLUMN_NAME AS CHAR) AS COLUMN_NAME, CAST(DATA_TYPE AS CHAR) AS DATA_TYPE, CAST(IS_NULLABLE AS CHAR) AS IS_NULLABLE, CAST(COLUMN_DEFAULT AS CHAR) AS COLUMN_DEFAULT, CAST(COLUMN_KEY AS CHAR) AS COLUMN_KEY FROM information_schema.columns WHERE TABLE_SCHEMA=DATABASE() AND TABLE_NAME=? ORDER BY ORDINAL_POSITION"
        )
        .bind(&table_name)
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;

        let columns: Vec<ColumnInfo> = col_rows
            .iter()
            .map(|r| {
                let is_nullable: String = r.try_get("IS_NULLABLE").unwrap_or_default();
                let column_key: String = r.try_get("COLUMN_KEY").unwrap_or_default();
                ColumnInfo {
                    name: r.try_get("COLUMN_NAME").unwrap_or_default(),
                    data_type: r.try_get("DATA_TYPE").unwrap_or_default(),
                    not_null: is_nullable.trim() == "NO",
                    default_value: r.try_get("COLUMN_DEFAULT").unwrap_or_default(),
                    is_primary_key: column_key.trim() == "PRI",
                }
            })
            .collect();

        tables.push(TableInfo {
            name: table_name,
            row_count: None,
            columns,
        });
    }

    let view_rows = sqlx::query(
        "SELECT TABLE_NAME FROM information_schema.views WHERE TABLE_SCHEMA=DATABASE() ORDER BY TABLE_NAME"
    )
    .fetch_all(pool)
    .await
    .map_err(|e| e.to_string())?;

    let views: Vec<String> = view_rows.iter().map(|r| r.get("TABLE_NAME")).collect();

    Ok(SchemaInfo {
        tables,
        views,
        indexes: vec![],
        triggers: vec![],
        schemas: vec![],
    })
}

pub async fn get_table_def(
    State(state): State<Arc<AppState>>,
    Path((id, table)): Path<(String, String)>,
) -> (StatusCode, Json<ApiResponse<TableDef>>) {
    tracing::info!(
        module = "handlers::schema",
        event = "get_table_def_start",
        connection_id = %id,
        table = %table,
        "Fetching table definition"
    );

    let conn_result = queries::get_connection(&state.db_pool, id.clone()).await;

    match conn_result {
        Ok(conn) => {
            let pool = match get_or_create_pool(&state, &conn).await {
                Ok(p) => p,
                Err(e) => {
                    tracing::error!(
                        module = "handlers::schema",
                        event = "pool_creation_failed",
                        connection_id = %id,
                        error = %e,
                        "Failed to create connection pool for table definition"
                    );
                    return (
                        StatusCode::BAD_REQUEST,
                        Json(ApiResponse::err(format!("Connection failed: {}", e))),
                    );
                }
            };

            let result = match pool {
                DynPool::Postgres(p) => get_postgres_table_def(&p, &table).await,
                DynPool::MySql(p) => get_mysql_table_def(&p, &table).await,
                DynPool::Sqlite(p) => get_sqlite_table_def(&p, &table).await,
            };

            match result {
                Ok(def) => {
                    tracing::info!(
                        module = "handlers::schema",
                        event = "get_table_def_success",
                        connection_id = %id,
                        table = %table,
                        column_count = def.columns.len(),
                        "Table definition fetched successfully"
                    );
                    (StatusCode::OK, Json(ApiResponse::ok(def)))
                }
                Err(e) => {
                    tracing::error!(
                        module = "handlers::schema",
                        event = "get_table_def_failed",
                        connection_id = %id,
                        table = %table,
                        error = %e,
                        "Failed to fetch table definition"
                    );
                    (StatusCode::BAD_REQUEST, Json(ApiResponse::err(e)))
                }
            }
        }
        Err(_) => {
            tracing::warn!(
                module = "handlers::schema",
                event = "connection_not_found",
                connection_id = %id,
                "Connection not found for table definition"
            );
            (
                StatusCode::NOT_FOUND,
                Json(ApiResponse::err("Connection not found")),
            )
        }
    }
}

async fn get_sqlite_table_def(
    pool: &sqlx::sqlite::SqlitePool,
    table: &str,
) -> Result<TableDef, String> {
    let safe_table = table.replace('"', "\"\"");
    let ddl: Option<String> =
        sqlx::query_scalar("SELECT sql FROM sqlite_master WHERE type='table' AND name=?")
            .bind(table)
            .fetch_optional(pool)
            .await
            .map_err(|e| e.to_string())?
            .unwrap_or_default();

    let col_rows = sqlx::query(&format!(
        "PRAGMA table_info('{}')",
        table.replace('\'', "''")
    ))
    .fetch_all(pool)
    .await
    .map_err(|e| e.to_string())?;

    let columns: Vec<ColumnInfo> = col_rows
        .iter()
        .map(|r| ColumnInfo {
            name: r.get("name"),
            data_type: r.get("type"),
            not_null: r.get::<i32, _>("notnull") == 1,
            default_value: r.get::<Option<String>, _>("dflt_value"),
            is_primary_key: r.get::<i32, _>("pk") > 0,
        })
        .collect();

    let idx_rows = sqlx::query(
        "SELECT name, tbl_name, sql FROM sqlite_master WHERE type='index' AND tbl_name=? ORDER BY name",
    )
    .bind(table)
    .fetch_all(pool)
    .await
    .map_err(|e| e.to_string())?;

    let indexes: Vec<IndexInfo> = idx_rows
        .iter()
        .map(|r| {
            let sql_val: String = r.get("sql");
            let is_unique = sql_val.to_uppercase().starts_with("CREATE UNIQUE");
            IndexInfo {
                name: r.get("name"),
                table_name: r.get("tbl_name"),
                columns: vec![],
                is_unique,
            }
        })
        .collect();

    let triggers: Vec<String> =
        sqlx::query_scalar("SELECT name FROM sqlite_master WHERE type='trigger' AND tbl_name=?")
            .bind(table)
            .fetch_all(pool)
            .await
            .map_err(|e| e.to_string())?;

    let row_count: Option<i64> =
        sqlx::query_scalar(&format!("SELECT COUNT(*) FROM \"{}\"", safe_table))
            .fetch_one(pool)
            .await
            .ok();

    Ok(TableDef {
        name: table.to_string(),
        ddl: ddl.unwrap_or_default(),
        columns,
        indexes,
        triggers,
        row_count,
    })
}

async fn get_postgres_table_def(
    pool: &sqlx::postgres::PgPool,
    table: &str,
) -> Result<TableDef, String> {
    let safe_table = table.replace('"', "\"\"");

    let col_rows = sqlx::query(
        "SELECT column_name, data_type, is_nullable, column_default, column_name IN (SELECT attname FROM pg_index i JOIN pg_attribute a ON a.attrelid = i.indrelid AND a.attnum = ANY(i.indkey) WHERE i.indrelid = $1::regclass AND i.indisprimary) AS is_primary FROM information_schema.columns WHERE table_schema='public' AND table_name=$1 ORDER BY ordinal_position",
    )
    .bind(table)
    .fetch_all(pool)
    .await
    .map_err(|e| e.to_string())?;

    let columns: Vec<ColumnInfo> = col_rows
        .iter()
        .map(|r| {
            let is_nullable: String = r.get("is_nullable");
            ColumnInfo {
                name: r.get("column_name"),
                data_type: r.get("data_type"),
                not_null: is_nullable == "NO",
                default_value: r.get("column_default"),
                is_primary_key: r.get("is_primary"),
            }
        })
        .collect();

    let idx_rows = sqlx::query(
        "SELECT indexrelid::regclass::text AS index_name, indisunique, array_to_string(array_agg(attname ORDER BY attnum), ', ') AS columns FROM pg_index i JOIN pg_attribute a ON a.attrelid = i.indrelid AND a.attnum = ANY(i.indkey) WHERE i.indrelid = $1::regclass AND NOT indisprimary GROUP BY indexrelid, indisunique",
    )
    .bind(table)
    .fetch_all(pool)
    .await
    .map_err(|e| e.to_string())?;

    let indexes: Vec<IndexInfo> = idx_rows
        .iter()
        .map(|r| IndexInfo {
            name: r.get("index_name"),
            table_name: table.to_string(),
            columns: r
                .get::<String, _>("columns")
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),
            is_unique: r.get("indisunique"),
        })
        .collect();

    let triggers: Vec<String> = sqlx::query_scalar(
        "SELECT tgname FROM pg_trigger WHERE tgrelid = $1::regclass AND NOT tgisinternal",
    )
    .bind(table)
    .fetch_all(pool)
    .await
    .map_err(|e| e.to_string())?;

    let row_count: Option<i64> =
        sqlx::query_scalar(&format!("SELECT COUNT(*) FROM \"{}\"", safe_table))
            .fetch_one(pool)
            .await
            .ok();

    Ok(TableDef {
        name: table.to_string(),
        ddl: String::new(),
        columns,
        indexes,
        triggers,
        row_count,
    })
}

async fn get_mysql_table_def(
    pool: &sqlx::mysql::MySqlPool,
    table: &str,
) -> Result<TableDef, String> {
    let ddl: Option<String> =
        sqlx::query_scalar(&format!("SHOW CREATE TABLE `{}`", table.replace('`', "``")))
            .fetch_optional(pool)
            .await
            .map_err(|e| e.to_string())?
            .unwrap_or_default();

    let col_rows = sqlx::query(
        "SELECT CAST(COLUMN_NAME AS CHAR) AS COLUMN_NAME, CAST(DATA_TYPE AS CHAR) AS DATA_TYPE, CAST(IS_NULLABLE AS CHAR) AS IS_NULLABLE, CAST(COLUMN_DEFAULT AS CHAR) AS COLUMN_DEFAULT, CAST(COLUMN_KEY AS CHAR) AS COLUMN_KEY FROM information_schema.columns WHERE TABLE_SCHEMA=DATABASE() AND TABLE_NAME=? ORDER BY ORDINAL_POSITION",
    )
    .bind(table)
    .fetch_all(pool)
    .await
    .map_err(|e| e.to_string())?;

    let columns: Vec<ColumnInfo> = col_rows
        .iter()
        .map(|r| {
            let is_nullable: String = r.get("IS_NULLABLE");
            let column_key: String = r.get("COLUMN_KEY");
            ColumnInfo {
                name: r.get("COLUMN_NAME"),
                data_type: r.get("DATA_TYPE"),
                not_null: is_nullable == "NO",
                default_value: r.get("COLUMN_DEFAULT"),
                is_primary_key: column_key == "PRI",
            }
        })
        .collect();

    let idx_rows = sqlx::query(
        "SELECT CAST(INDEX_NAME AS CHAR) AS INDEX_NAME, CAST(NON_UNIQUE AS CHAR) AS NON_UNIQUE, CAST(COLUMN_NAME AS CHAR) AS COLUMN_NAME FROM information_schema.statistics WHERE TABLE_SCHEMA=DATABASE() AND TABLE_NAME=? AND INDEX_NAME != 'PRIMARY' ORDER BY INDEX_NAME, SEQ_IN_INDEX",
    )
    .bind(table)
    .fetch_all(pool)
    .await
    .map_err(|e| e.to_string())?;

    let mut index_map: std::collections::HashMap<String, (bool, Vec<String>)> =
        std::collections::HashMap::new();
    for row in idx_rows {
        let idx_name: String = row.get("INDEX_NAME");
        let non_unique: String = row.get("NON_UNIQUE");
        let col_name: String = row.get("COLUMN_NAME");
        let entry = index_map
            .entry(idx_name)
            .or_insert((non_unique == "0", Vec::new()));
        entry.1.push(col_name);
    }

    let indexes: Vec<IndexInfo> = index_map
        .into_iter()
        .map(|(name, (is_unique, cols))| IndexInfo {
            name,
            table_name: table.to_string(),
            columns: cols,
            is_unique,
        })
        .collect();

    let triggers: Vec<String> = sqlx::query_scalar(&format!(
        "SELECT CAST(TRIGGER_NAME AS CHAR) FROM information_schema.triggers WHERE TRIGGER_SCHEMA=DATABASE() AND EVENT_OBJECT_TABLE='{}'",
        table.replace('\'', "''")
    ))
    .fetch_all(pool)
    .await
    .map_err(|e| e.to_string())?;

    let row_count: Option<i64> = sqlx::query_scalar(&format!(
        "SELECT COUNT(*) FROM `{}`",
        table.replace('`', "``")
    ))
    .fetch_one(pool)
    .await
    .ok();

    Ok(TableDef {
        name: table.to_string(),
        ddl: ddl.unwrap_or_default(),
        columns,
        indexes,
        triggers,
        row_count,
    })
}
