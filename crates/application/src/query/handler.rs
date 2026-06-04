use std::hash::{Hash, Hasher};
use std::sync::Arc;
use sql_admin_domain::connection::repository::ConnectionRepository;
use sql_admin_domain::connection::value_objects::DatabaseType;
use sql_admin_domain::query::value_objects::QueryText;
use sql_admin_domain::shared::application_error::ApplicationError;
use sql_admin_domain::shared::crypto::EncryptionService;
use sql_admin_domain::shared::event::{DomainEvent, EventBus};
use sql_admin_domain::shared::pool::{PoolFactory, QueryResult, TableDefinition, TableInfo};

use crate::connection_pool_service::ConnectionPoolService;
use crate::dto::{ExecuteQueryRequest, GetTableDataRequest};
use crate::utils::is_safe_identifier;

pub struct QueryHandler {
    pool_service: ConnectionPoolService,
    event_bus: Arc<dyn EventBus>,
}

impl QueryHandler {
    pub fn new(
        conn_repo: Arc<dyn ConnectionRepository>,
        pool_factory: Arc<dyn PoolFactory>,
        crypto: Arc<dyn EncryptionService>,
        event_bus: Arc<dyn EventBus>,
    ) -> Self {
        let pool_service = ConnectionPoolService::new(conn_repo, pool_factory, crypto);
        Self {
            pool_service,
            event_bus,
        }
    }

    pub async fn execute(&self, cmd: ExecuteQueryRequest) -> Result<QueryResult, ApplicationError> {
        let _query_text = QueryText::new(&cmd.query)?;

        let executor = self.pool_service.get_executor(&cmd.connection_id).await?;

        let result: QueryResult = if _query_text.is_select_query() {
            executor.execute_query(&cmd.query).await.map_err(ApplicationError::Infrastructure)?
        } else {
            let dml_result = executor.execute_dml(&cmd.query).await?;
            QueryResult {
                columns: vec!["rows_affected".to_string()],
                rows: vec![vec![serde_json::Value::Number(dml_result.rows_affected.into())]],
                rows_affected: Some(dml_result.rows_affected),
                execution_time_ms: Some(dml_result.execution_time_ms),
            }
        };

        let mut hasher = std::hash::DefaultHasher::new();
        cmd.query.hash(&mut hasher);
        let query_hash = format!("{:x}", hasher.finish());

        let event = DomainEvent::QueryExecuted {
            connection_id: cmd.connection_id.clone(),
            connection_name: String::new(), // Will be filled by consumer or left empty
            query_text: cmd.query.clone(),
            query_hash,
            execution_time_ms: result.execution_time_ms.unwrap_or(0),
            success: true,
            timestamp: chrono::Utc::now(),
        };
        if let Err(e) = self.event_bus.publish(event).await {
            tracing::warn!(
                module = "query_handler",
                event = "event_publish_failed",
                error = %e,
                "Failed to publish query executed event"
            );
        }

        Ok(result)
    }

    pub async fn get_table_data(&self, cmd: GetTableDataRequest) -> Result<QueryResult, ApplicationError> {
        let (executor, db_type) = self.pool_service.get_executor_with_type(&cmd.connection_id).await?;

        if !is_safe_identifier(&cmd.table) {
            return Err(ApplicationError::Validation(format!(
                "Invalid table name: {}",
                cmd.table
            )));
        }

        if cmd.limit < 0 {
            return Err(ApplicationError::Validation(
                "Limit must be non-negative".to_string(),
            ));
        }
        if cmd.offset < 0 {
            return Err(ApplicationError::Validation(
                "Offset must be non-negative".to_string(),
            ));
        }

        // Cap limit to prevent excessive memory usage
        let limit = if cmd.limit == 0 { 100 } else { cmd.limit.min(10000) };

        let sql = match db_type {
            DatabaseType::Mysql => format!(
                "SELECT * FROM `{}` LIMIT {} OFFSET {}",
                cmd.table, limit, cmd.offset
            ),
            _ => format!(
                "SELECT * FROM \"{}\" LIMIT {} OFFSET {}",
                cmd.table, limit, cmd.offset
            ),
        };

        executor.execute_query(&sql).await.map_err(Into::into)
    }

    pub async fn get_schema(&self, connection_id: &str) -> Result<Vec<TableInfo>, ApplicationError> {
        let executor = self.pool_service.get_executor(connection_id).await?;
        executor.get_schema().await.map_err(Into::into)
    }

    pub async fn get_schema_with_columns(&self, connection_id: &str) -> Result<Vec<TableDefinition>, ApplicationError> {
        let executor = self.pool_service.get_executor(connection_id).await?;
        executor.get_all_table_definitions().await.map_err(Into::into)
    }

    pub async fn get_table_definition(
        &self,
        connection_id: &str,
        table_name: &str,
    ) -> Result<TableDefinition, ApplicationError> {
        let executor = self.pool_service.get_executor(connection_id).await?;

        if !is_safe_identifier(table_name) {
            return Err(ApplicationError::Validation(format!(
                "Invalid table name: {}",
                table_name
            )));
        }

        executor.get_table_definition(table_name).await.map_err(Into::into)
    }
}
