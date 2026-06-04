use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "PascalCase")]
pub enum DatabaseType {
    Postgres,
    Mysql,
    Sqlite,
    Redb,
}

impl std::fmt::Display for DatabaseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DatabaseType::Postgres => write!(f, "postgres"),
            DatabaseType::Mysql => write!(f, "mysql"),
            DatabaseType::Sqlite => write!(f, "sqlite"),
            DatabaseType::Redb => write!(f, "redb"),
        }
    }
}

impl TryFrom<&str> for DatabaseType {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "postgres" => Ok(DatabaseType::Postgres),
            "mysql" => Ok(DatabaseType::Mysql),
            "sqlite" => Ok(DatabaseType::Sqlite),
            "redb" => Ok(DatabaseType::Redb),
            other => Err(format!("Unknown database type: {}", other)),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize, Clone, Debug)]
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
    pub execution_time_ms: Option<u64>,
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
pub struct DeleteConnectionRequest {
    pub id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetTableDataRequest {
    pub connection_id: String,
    pub table: String,
    pub limit: i64,
    pub offset: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EditRowRequest {
    pub connection_id: String,
    pub table_name: String,
    pub primary_key_column: String,
    pub primary_key_value: serde_json::Value,
    pub column: String,
    pub new_value: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeleteRowRequest {
    pub connection_id: String,
    pub table_name: String,
    pub primary_key_column: String,
    pub primary_key_value: serde_json::Value,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InsertRowRequest {
    pub connection_id: String,
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

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum TransactionMode {
    /// Roll back all statements if any fails
    AllOrNothing,
    /// Continue executing on error, collect errors
    #[default]
    ContinueOnError,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ImportSqlRequest {
    pub connection_id: String,
    pub sql_content: String,
    #[serde(default)]
    pub transaction_mode: TransactionMode,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ImportResult {
    pub statements_executed: u32,
    pub errors: Vec<String>,
    pub execution_time_ms: Option<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RedbTableSummary {
    pub name: String,
    pub key_count: u64,
    pub total_value_bytes: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RedbKeyValue {
    pub key: String,
    pub value: serde_json::Value,
    pub value_type: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RedbKeyList {
    pub keys: Vec<RedbKeyValue>,
    pub total: u64,
    pub has_more: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RedbQueryRequest {
    pub connection_id: String,
    pub table: String,
    pub key_prefix: Option<String>,
    pub key_pattern: Option<String>,
    pub limit: u64,
    pub offset: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RedbEditRequest {
    pub connection_id: String,
    pub table: String,
    pub key: String,
    pub new_value: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RedbBatchDeleteRequest {
    pub connection_id: String,
    pub table: String,
    pub keys: Vec<String>,
}
