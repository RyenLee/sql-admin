use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[cfg(feature = "sqlx")]
impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        AppError::Database(e.to_string())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Connection {
    pub id: String,
    pub name: String,
    pub database_type: DatabaseType,
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum DatabaseType {
    Postgres,
    Mysql,
    Sqlite,
}

impl std::fmt::Display for DatabaseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DatabaseType::Postgres => write!(f, "postgres"),
            DatabaseType::Mysql => write!(f, "mysql"),
            DatabaseType::Sqlite => write!(f, "sqlite"),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateConnectionRequest {
    pub name: String,
    pub database_type: DatabaseType,
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateConnectionRequest {
    pub name: Option<String>,
    pub host: Option<String>,
    pub port: Option<u16>,
    pub database: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QueryResult {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rows_affected: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_time_ms: Option<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExecuteQueryRequest {
    pub connection_id: String,
    pub query: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SchemaInfo {
    pub tables: Vec<TableInfo>,
    pub views: Vec<String>,
    pub indexes: Vec<IndexInfo>,
    pub triggers: Vec<String>,
    pub schemas: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TableInfo {
    pub name: String,
    pub row_count: Option<i64>,
    pub columns: Vec<ColumnInfo>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ColumnInfo {
    pub name: String,
    pub data_type: String,
    pub not_null: bool,
    pub default_value: Option<String>,
    pub is_primary_key: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IndexInfo {
    pub name: String,
    pub table_name: String,
    pub columns: Vec<String>,
    pub is_unique: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn err(error: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error.into()),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QueryHistory {
    pub id: String,
    pub connection_id: String,
    pub connection_name: String,
    pub query_text: String,
    pub execution_time_ms: Option<i64>,
    pub rows_count: Option<i64>,
    pub success: bool,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SaveQueryHistoryRequest {
    pub connection_id: String,
    pub connection_name: String,
    pub query_text: String,
    pub execution_time_ms: Option<u64>,
    pub rows_count: Option<i64>,
    pub success: bool,
    pub error_message: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EditRowRequest {
    pub table_name: String,
    pub primary_key_column: String,
    pub primary_key_value: serde_json::Value,
    pub column: String,
    pub new_value: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeleteRowRequest {
    pub table_name: String,
    pub primary_key_column: String,
    pub primary_key_value: serde_json::Value,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InsertRowRequest {
    pub table_name: String,
    pub columns: Vec<String>,
    pub values: Vec<serde_json::Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TableDef {
    pub name: String,
    pub ddl: String,
    pub columns: Vec<ColumnInfo>,
    pub indexes: Vec<IndexInfo>,
    pub triggers: Vec<String>,
    pub row_count: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ImportSqlRequest {
    pub sql_content: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ImportResult {
    pub statements_executed: u32,
    pub errors: Vec<String>,
    pub execution_time_ms: Option<u64>,
}

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(String),

    #[error("Connection not found")]
    ConnectionNotFound,

    #[error("Invalid input: {0}")]
    Validation(String),

    #[error("Query execution failed: {0}")]
    QueryFailed(String),

    #[error("Internal server error")]
    InternalError,
}
