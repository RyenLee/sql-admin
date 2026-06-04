use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::connection::value_objects::DatabaseType;
use crate::shared::infrastructure_error::InfrastructureError;

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
pub struct DmlResult {
    pub rows_affected: u64,
    pub execution_time_ms: u64,
}

#[async_trait]
pub trait QueryExecutor: Send + Sync {
    async fn execute_query(&self, sql: &str) -> Result<QueryResult, InfrastructureError>;
    async fn execute_dml(&self, sql: &str) -> Result<DmlResult, InfrastructureError>;
    async fn get_schema(&self) -> Result<Vec<TableInfo>, InfrastructureError>;
    async fn get_table_definition(&self, table_name: &str) -> Result<TableDefinition, InfrastructureError>;
    async fn get_all_table_definitions(&self) -> Result<Vec<TableDefinition>, InfrastructureError> {
        let schema = self.get_schema().await?;
        let mut defs = Vec::with_capacity(schema.len());
        for t in &schema {
            if let Ok(def) = self.get_table_definition(&t.name).await {
                defs.push(def);
            }
        }
        Ok(defs)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TableInfo {
    pub name: String,
    pub table_type: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TableDefinition {
    pub table_name: String,
    pub columns: Vec<ColumnInfo>,
    pub primary_keys: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ColumnInfo {
    pub name: String,
    pub data_type: String,
    pub nullable: bool,
    pub is_primary_key: bool,
}

#[async_trait]
#[allow(clippy::too_many_arguments)]
pub trait PoolFactory: Send + Sync {
    async fn create_pool(
        &self,
        connection_id: &str,
        database_type: &DatabaseType,
        host: &str,
        port: u16,
        database: &str,
        username: &str,
        password: &str,
    ) -> Result<Arc<dyn QueryExecutor>, InfrastructureError>;

    async fn invalidate_pool(&self, connection_id: &str);
}

#[async_trait]
pub trait RedbExecutor: Send + Sync {
    async fn list_redb_tables(
        &self,
        connection_id: &str,
    ) -> Result<Vec<(String, u64, u64)>, InfrastructureError>;

    async fn query_redb_keys(
        &self,
        connection_id: &str,
        table: &str,
        key_prefix: Option<&str>,
        limit: u64,
        offset: u64,
    ) -> Result<Vec<(String, serde_json::Value)>, InfrastructureError>;

    async fn edit_redb_key(
        &self,
        connection_id: &str,
        table: &str,
        key: &str,
        value: serde_json::Value,
    ) -> Result<(), InfrastructureError>;

    async fn batch_delete_redb_keys(
        &self,
        connection_id: &str,
        table: &str,
        keys: &[String],
    ) -> Result<u64, InfrastructureError>;
}