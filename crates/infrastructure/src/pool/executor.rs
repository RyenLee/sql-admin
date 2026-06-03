use async_trait::async_trait;
use std::sync::Arc;
use sql_admin_domain::shared::infrastructure_error::InfrastructureError;
use sql_admin_domain::shared::pool::{
    ColumnInfo, DmlResult, QueryExecutor, QueryResult,
    TableDefinition, TableInfo,
};

use super::factory::DynPool;

pub struct DynQueryExecutor {
    pool: DynPool,
}

impl DynQueryExecutor {
    pub fn new(pool: DynPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl QueryExecutor for DynQueryExecutor {
    async fn execute_query(&self, sql: &str) -> Result<QueryResult, InfrastructureError> {
        match &self.pool {
            DynPool::Postgres(p) => execute_pg_query(p, sql).await,
            DynPool::Mysql(p) => execute_mysql_query(p, sql).await,
            DynPool::Sqlite(p) => execute_sqlite_query(p, sql).await,
            DynPool::Redb(_) => Err(InfrastructureError::DatabaseError(
                "Redb does not support SQL queries".to_string(),
            )),
        }
    }

    async fn execute_dml(&self, sql: &str) -> Result<DmlResult, InfrastructureError> {
        match &self.pool {
            DynPool::Postgres(p) => execute_pg_dml(p, sql).await,
            DynPool::Mysql(p) => execute_mysql_dml(p, sql).await,
            DynPool::Sqlite(p) => execute_sqlite_dml(p, sql).await,
            DynPool::Redb(_) => Err(InfrastructureError::DatabaseError(
                "Redb does not support DML operations".to_string(),
            )),
        }
    }

    async fn get_schema(&self) -> Result<Vec<TableInfo>, InfrastructureError> {
        match &self.pool {
            DynPool::Postgres(p) => get_pg_schema(p).await,
            DynPool::Mysql(p) => get_mysql_schema(p).await,
            DynPool::Sqlite(p) => get_sqlite_schema(p).await,
            DynPool::Redb(db) => get_redb_schema(db).await,
        }
    }

    async fn get_table_definition(&self, table_name: &str) -> Result<TableDefinition, InfrastructureError> {
        match &self.pool {
            DynPool::Postgres(p) => get_pg_table_def(p, table_name).await,
            DynPool::Mysql(p) => get_mysql_table_def(p, table_name).await,
            DynPool::Sqlite(p) => get_sqlite_table_def(p, table_name).await,
            DynPool::Redb(db) => get_redb_table_def(db, table_name).await,
        }
    }
}

async fn execute_pg_query(pool: &sqlx::PgPool, sql: &str) -> Result<QueryResult, InfrastructureError> {
    use sqlx::{AssertSqlSafe, Column, Row};
    let start = std::time::Instant::now();
    let rows = sqlx::query(AssertSqlSafe(sql.to_owned()))
        .fetch_all(pool)
        .await
        .map_err(|e| InfrastructureError::DatabaseError(format!("Postgres query failed: {}", e)))?;

    let columns: Vec<String> = if rows.is_empty() {
        Vec::new()
    } else {
        rows[0].columns().iter().map(|c| c.name().to_string()).collect()
    };

    let data: Vec<Vec<serde_json::Value>> = rows
        .iter()
        .map(|row| {
            columns
                .iter()
                .map(|col| {
                    let idx = row.columns().iter().position(|c| c.name() == col);
                    match idx {
                        Some(i) => value_from_pg_row(row, i),
                        None => serde_json::Value::Null,
                    }
                })
                .collect()
        })
        .collect();

    Ok(QueryResult {
        columns,
        rows: data,
        rows_affected: None,
        execution_time_ms: Some(start.elapsed().as_millis() as u64),
    })
}

async fn execute_pg_dml(pool: &sqlx::PgPool, sql: &str) -> Result<DmlResult, InfrastructureError> {
    use sqlx::AssertSqlSafe;
    let start = std::time::Instant::now();
    let result = sqlx::query(AssertSqlSafe(sql.to_owned()))
        .execute(pool)
        .await
        .map_err(|e| InfrastructureError::DatabaseError(format!("Postgres DML failed: {}", e)))?;

    Ok(DmlResult {
        rows_affected: result.rows_affected(),
        execution_time_ms: start.elapsed().as_millis() as u64,
    })
}

async fn execute_mysql_query(pool: &sqlx::MySqlPool, sql: &str) -> Result<QueryResult, InfrastructureError> {
    use sqlx::{AssertSqlSafe, Column, Row};
    let start = std::time::Instant::now();
    let rows = sqlx::query(AssertSqlSafe(sql.to_owned()))
        .fetch_all(pool)
        .await
        .map_err(|e| InfrastructureError::DatabaseError(format!("MySQL query failed: {}", e)))?;

    let columns: Vec<String> = if rows.is_empty() {
        Vec::new()
    } else {
        rows[0].columns().iter().map(|c| c.name().to_string()).collect()
    };

    let data: Vec<Vec<serde_json::Value>> = rows
        .iter()
        .map(|row| {
            columns
                .iter()
                .map(|col| {
                    let idx = row.columns().iter().position(|c| c.name() == col);
                    match idx {
                        Some(i) => value_from_mysql_row(row, i),
                        None => serde_json::Value::Null,
                    }
                })
                .collect()
        })
        .collect();

    Ok(QueryResult {
        columns,
        rows: data,
        rows_affected: None,
        execution_time_ms: Some(start.elapsed().as_millis() as u64),
    })
}

async fn execute_mysql_dml(pool: &sqlx::MySqlPool, sql: &str) -> Result<DmlResult, InfrastructureError> {
    use sqlx::AssertSqlSafe;
    let start = std::time::Instant::now();
    let result = sqlx::query(AssertSqlSafe(sql.to_owned()))
        .execute(pool)
        .await
        .map_err(|e| InfrastructureError::DatabaseError(format!("MySQL DML failed: {}", e)))?;

    Ok(DmlResult {
        rows_affected: result.rows_affected(),
        execution_time_ms: start.elapsed().as_millis() as u64,
    })
}

async fn execute_sqlite_query(pool: &sqlx::SqlitePool, sql: &str) -> Result<QueryResult, InfrastructureError> {
    use sqlx::{AssertSqlSafe, Column, Row};
    let start = std::time::Instant::now();
    let rows = sqlx::query(AssertSqlSafe(sql.to_owned()))
        .fetch_all(pool)
        .await
        .map_err(|e| InfrastructureError::DatabaseError(format!("SQLite query failed: {}", e)))?;

    let columns: Vec<String> = if rows.is_empty() {
        Vec::new()
    } else {
        rows[0].columns().iter().map(|c| c.name().to_string()).collect()
    };

    let data: Vec<Vec<serde_json::Value>> = rows
        .iter()
        .map(|row| {
            columns
                .iter()
                .map(|col| {
                    let idx = row.columns().iter().position(|c| c.name() == col);
                    match idx {
                        Some(i) => value_from_sqlite_row(row, i),
                        None => serde_json::Value::Null,
                    }
                })
                .collect()
        })
        .collect();

    Ok(QueryResult {
        columns,
        rows: data,
        rows_affected: None,
        execution_time_ms: Some(start.elapsed().as_millis() as u64),
    })
}

async fn execute_sqlite_dml(pool: &sqlx::SqlitePool, sql: &str) -> Result<DmlResult, InfrastructureError> {
    use sqlx::AssertSqlSafe;
    let start = std::time::Instant::now();
    let result = sqlx::query(AssertSqlSafe(sql.to_owned()))
        .execute(pool)
        .await
        .map_err(|e| InfrastructureError::DatabaseError(format!("SQLite DML failed: {}", e)))?;

    Ok(DmlResult {
        rows_affected: result.rows_affected(),
        execution_time_ms: start.elapsed().as_millis() as u64,
    })
}

async fn get_pg_schema(pool: &sqlx::PgPool) -> Result<Vec<TableInfo>, InfrastructureError> {
    use sqlx::Row;
    let rows = sqlx::query(
        "SELECT table_name, table_type FROM information_schema.tables WHERE table_schema = 'public' ORDER BY table_name",
    )
    .fetch_all(pool)
    .await
    .map_err(|e| InfrastructureError::DatabaseError(format!("Failed to get schema: {}", e)))?;

    Ok(rows
        .iter()
        .map(|row| TableInfo {
            name: row.get::<String, _>(0),
            table_type: row.get::<String, _>(1),
        })
        .collect())
}

async fn get_mysql_schema(pool: &sqlx::MySqlPool) -> Result<Vec<TableInfo>, InfrastructureError> {
    use sqlx::Row;
    let rows = sqlx::query("SHOW TABLES")
        .fetch_all(pool)
        .await
        .map_err(|e| InfrastructureError::DatabaseError(format!("Failed to get schema: {}", e)))?;

    Ok(rows
        .iter()
        .map(|row| TableInfo {
            name: row.get::<String, _>(0),
            table_type: "TABLE".to_string(),
        })
        .collect())
}

async fn get_sqlite_schema(pool: &sqlx::SqlitePool) -> Result<Vec<TableInfo>, InfrastructureError> {
    use sqlx::Row;
    let rows = sqlx::query(
        "SELECT name, type FROM sqlite_master WHERE type IN ('table', 'view') AND name NOT LIKE 'sqlite_%' ORDER BY name",
    )
    .fetch_all(pool)
    .await
    .map_err(|e| InfrastructureError::DatabaseError(format!("Failed to get schema: {}", e)))?;

    Ok(rows
        .iter()
        .map(|row| TableInfo {
            name: row.get::<String, _>(0),
            table_type: row.get::<String, _>(1),
        })
        .collect())
}

async fn get_redb_schema(_db: &Arc<redb::Database>) -> Result<Vec<TableInfo>, InfrastructureError> {
    Ok(Vec::new())
}

async fn get_pg_table_def(pool: &sqlx::PgPool, table_name: &str) -> Result<TableDefinition, InfrastructureError> {
    use sqlx::Row;
    let cols = sqlx::query(
        "SELECT column_name, data_type, is_nullable, CASE WHEN pk.column_name IS NOT NULL THEN true ELSE false END AS is_pk
         FROM information_schema.columns c
         LEFT JOIN (
             SELECT ku.column_name FROM information_schema.table_constraints tc
             JOIN information_schema.key_column_usage ku ON tc.constraint_name = ku.constraint_name
             WHERE tc.constraint_type = 'PRIMARY KEY' AND tc.table_name = $1
         ) pk ON c.column_name = pk.column_name
         WHERE c.table_name = $1 ORDER BY c.ordinal_position",
    )
    .bind(table_name)
    .fetch_all(pool)
    .await
    .map_err(|e| InfrastructureError::DatabaseError(format!("Failed to get table definition: {}", e)))?;

    let columns: Vec<ColumnInfo> = cols
        .iter()
        .map(|row| ColumnInfo {
            name: row.get::<String, _>(0),
            data_type: row.get::<String, _>(1),
            nullable: row.get::<String, _>(2) == "YES",
            is_primary_key: row.get::<bool, _>(3),
        })
        .collect();

    let primary_keys: Vec<String> = columns
        .iter()
        .filter(|c| c.is_primary_key)
        .map(|c| c.name.clone())
        .collect();

    Ok(TableDefinition {
        table_name: table_name.to_string(),
        columns,
        primary_keys,
    })
}

fn sanitize_identifier(name: &str) -> Result<String, InfrastructureError> {
    if name.is_empty() {
        return Err(InfrastructureError::DatabaseError("Identifier cannot be empty".to_string()));
    }
    if name.contains('\0') {
        return Err(InfrastructureError::DatabaseError("Identifier contains null byte".to_string()));
    }
    let sanitized = name.replace('"', "\"\"");
    Ok(format!("\"{}\"", sanitized))
}

async fn get_mysql_table_def(pool: &sqlx::MySqlPool, table_name: &str) -> Result<TableDefinition, InfrastructureError> {
    use sqlx::{AssertSqlSafe, Row};
    let safe_name = sanitize_identifier(table_name)?;
    let cols = sqlx::query(AssertSqlSafe(format!("DESCRIBE {}", safe_name)))
        .fetch_all(pool)
        .await
        .map_err(|e| InfrastructureError::DatabaseError(format!("Failed to get table definition: {}", e)))?;

    let columns: Vec<ColumnInfo> = cols
        .iter()
        .map(|row| {
            let key = row.get::<String, _>(3);
            ColumnInfo {
                name: row.get::<String, _>(0),
                data_type: row.get::<String, _>(1),
                nullable: row.get::<String, _>(2) == "YES",
                is_primary_key: key == "PRI",
            }
        })
        .collect();

    let primary_keys: Vec<String> = columns
        .iter()
        .filter(|c| c.is_primary_key)
        .map(|c| c.name.clone())
        .collect();

    Ok(TableDefinition {
        table_name: table_name.to_string(),
        columns,
        primary_keys,
    })
}

async fn get_sqlite_table_def(pool: &sqlx::SqlitePool, table_name: &str) -> Result<TableDefinition, InfrastructureError> {
    use sqlx::{AssertSqlSafe, Row};
    let safe_name = sanitize_identifier(table_name)?;
    let cols = sqlx::query(AssertSqlSafe(format!("PRAGMA table_info({})", safe_name)))
        .fetch_all(pool)
        .await
        .map_err(|e| InfrastructureError::DatabaseError(format!("Failed to get table definition: {}", e)))?;

    let columns: Vec<ColumnInfo> = cols
        .iter()
        .map(|row| ColumnInfo {
            name: row.get::<String, _>(1),
            data_type: row.get::<String, _>(2),
            nullable: !row.get::<bool, _>(3),
            is_primary_key: row.get::<i64, _>(5) != 0,
        })
        .collect();

    let primary_keys: Vec<String> = columns
        .iter()
        .filter(|c| c.is_primary_key)
        .map(|c| c.name.clone())
        .collect();

    Ok(TableDefinition {
        table_name: table_name.to_string(),
        columns,
        primary_keys,
    })
}

async fn get_redb_table_def(_db: &Arc<redb::Database>, _table_name: &str) -> Result<TableDefinition, InfrastructureError> {
    Ok(TableDefinition {
        table_name: _table_name.to_string(),
        columns: Vec::new(),
        primary_keys: Vec::new(),
    })
}

fn value_from_pg_row(row: &sqlx::postgres::PgRow, idx: usize) -> serde_json::Value {
    use sqlx::{Column, Row, TypeInfo};
    let type_name = row.columns()[idx].type_info().name();
    match type_name {
        "INT2" | "INT4" | "INT8" => row.try_get::<i64, _>(idx).map(serde_json::Value::from).unwrap_or(serde_json::Value::Null),
        "FLOAT4" | "FLOAT8" => row.try_get::<f64, _>(idx).map(serde_json::Value::from).unwrap_or(serde_json::Value::Null),
        "BOOL" => row.try_get::<bool, _>(idx).map(serde_json::Value::from).unwrap_or(serde_json::Value::Null),
        "VARCHAR" | "TEXT" | "CHAR" | "BPCHAR" | "NAME" => row.try_get::<String, _>(idx).map(serde_json::Value::from).unwrap_or(serde_json::Value::Null),
        _ => row.try_get::<String, _>(idx).map(serde_json::Value::from).unwrap_or(serde_json::Value::Null),
    }
}

fn value_from_mysql_row(row: &sqlx::mysql::MySqlRow, idx: usize) -> serde_json::Value {
    use sqlx::{Column, Row, TypeInfo};
    let type_name = row.columns()[idx].type_info().name().to_lowercase();
    match type_name.as_str() {
        "int" | "tinyint" | "smallint" | "mediumint" | "bigint" => row.try_get::<i64, _>(idx).map(serde_json::Value::from).unwrap_or(serde_json::Value::Null),
        "float" | "double" | "decimal" => row.try_get::<f64, _>(idx).map(serde_json::Value::from).unwrap_or(serde_json::Value::Null),
        "varchar" | "char" | "text" | "mediumtext" | "longtext" | "tinytext" => row.try_get::<String, _>(idx).map(serde_json::Value::from).unwrap_or(serde_json::Value::Null),
        _ => row.try_get::<String, _>(idx).map(serde_json::Value::from).unwrap_or(serde_json::Value::Null),
    }
}

fn value_from_sqlite_row(row: &sqlx::sqlite::SqliteRow, idx: usize) -> serde_json::Value {
    use sqlx::{Column, Row, TypeInfo};
    let type_name = row.columns()[idx].type_info().name().to_uppercase();
    tracing::debug!(
        column_index = idx,
        column_name = row.columns()[idx].name(),
        type_name = %type_name,
        "Extracting SQLite value"
    );
    match type_name.as_str() {
        "INTEGER" | "INT" | "BIGINT" | "SMALLINT" | "TINYINT" => {
            row.try_get::<i64, _>(idx).map(serde_json::Value::from).unwrap_or(serde_json::Value::Null)
        }
        "REAL" | "FLOAT" | "DOUBLE" | "NUMERIC" | "DECIMAL" => {
            row.try_get::<f64, _>(idx).map(serde_json::Value::from).unwrap_or(serde_json::Value::Null)
        }
        "BOOLEAN" | "BOOL" => {
            row.try_get::<bool, _>(idx).map(serde_json::Value::from).unwrap_or(serde_json::Value::Null)
        }
        "TEXT" | "VARCHAR" | "CHAR" | "CLOB" | "NVARCHAR" | "NCHAR" => {
            row.try_get::<String, _>(idx).map(serde_json::Value::from).unwrap_or(serde_json::Value::Null)
        }
        "BLOB" | "BINARY" | "VARBINARY" => {
            row.try_get::<Vec<u8>, _>(idx)
                .map(|bytes| {
                    serde_json::Value::String(format!("[BLOB: {} bytes]", bytes.len()))
                })
                .unwrap_or(serde_json::Value::Null)
        }
        // "NULL" type name means the column has no declared type (flexible typing).
        // This does NOT mean the value is NULL — use fallback extraction instead.
        "NULL" | "" | _ => {
            // Fallback: try as String first, then as i64, then as f64
            row.try_get::<String, _>(idx)
                .map(serde_json::Value::from)
                .or_else(|_| row.try_get::<i64, _>(idx).map(serde_json::Value::from))
                .or_else(|_| row.try_get::<f64, _>(idx).map(serde_json::Value::from))
                .unwrap_or(serde_json::Value::Null)
        }
    }
}