use sql_admin_domain::connection::repository::ConnectionRepository;
use sql_admin_domain::connection::value_objects::DatabaseType;
use sql_admin_domain::redb::value_objects::{RedbKey, RedbValue};
use sql_admin_domain::shared::application_error::ApplicationError;
use sql_admin_domain::shared::crypto::EncryptionService;
use sql_admin_domain::shared::pool::{PoolFactory, RedbExecutor};
use std::sync::Arc;

use crate::connection_pool_service::ConnectionPoolService;

pub struct RedbHandler {
    conn_repo: Arc<dyn ConnectionRepository>,
    pool_service: ConnectionPoolService,
    redb_executor: Arc<dyn RedbExecutor>,
}

impl RedbHandler {
    pub fn new(
        conn_repo: Arc<dyn ConnectionRepository>,
        pool_factory: Arc<dyn PoolFactory>,
        redb_executor: Arc<dyn RedbExecutor>,
        crypto: Arc<dyn EncryptionService>,
    ) -> Self {
        let pool_service = ConnectionPoolService::new(conn_repo.clone(), pool_factory, crypto);
        Self {
            conn_repo,
            pool_service,
            redb_executor,
        }
    }

    pub async fn list_tables(
        &self,
        connection_id: &str,
    ) -> Result<Vec<(String, u64)>, ApplicationError> {
        self.ensure_pool(connection_id).await?;
        let tables = self.redb_executor.list_redb_tables(connection_id).await?;
        Ok(tables)
    }

    pub async fn query_keys(
        &self,
        connection_id: &str,
        table_name: &str,
        key_prefix: Option<&str>,
        limit: u64,
        offset: u64,
    ) -> Result<Vec<(RedbKey, RedbValue)>, ApplicationError> {
        self.ensure_pool(connection_id).await?;

        let results = self
            .redb_executor
            .query_redb_keys(connection_id, table_name, key_prefix, limit, offset)
            .await?;

        Ok(results
            .into_iter()
            .map(|(k, v)| (RedbKey::new(k), RedbValue::new(v)))
            .collect())
    }

    pub async fn edit_key(
        &self,
        connection_id: &str,
        table_name: &str,
        key: RedbKey,
        new_value: RedbValue,
    ) -> Result<(), ApplicationError> {
        self.ensure_pool(connection_id).await?;

        self.redb_executor
            .edit_redb_key(
                connection_id,
                table_name,
                key.as_str(),
                new_value.as_json().clone(),
            )
            .await?;

        Ok(())
    }

    pub async fn batch_delete_keys(
        &self,
        connection_id: &str,
        table_name: &str,
        keys: Vec<RedbKey>,
    ) -> Result<u64, ApplicationError> {
        self.ensure_pool(connection_id).await?;

        let key_strs: Vec<String> = keys.iter().map(|k| k.as_str().to_string()).collect();
        let count = self
            .redb_executor
            .batch_delete_redb_keys(connection_id, table_name, &key_strs)
            .await?;

        Ok(count)
    }

    /// Ensure the connection pool is created and cached before redb operations.
    /// This is needed because redb operations rely on the pool cache,
    /// but the pool may not exist yet if no prior query was executed.
    /// Also validates that the connection is of Redb type.
    async fn ensure_pool(&self, connection_id: &str) -> Result<(), ApplicationError> {
        let conn = self
            .conn_repo
            .find_by_id(connection_id)
            .await?
            .ok_or_else(|| {
                ApplicationError::NotFound(format!("Connection not found: {}", connection_id))
            })?;

        if conn.database_type() != &DatabaseType::Redb {
            return Err(ApplicationError::Validation(
                "Only Redb connections support this operation".to_string(),
            ));
        }

        let _ = self.pool_service.get_executor(connection_id).await?;
        Ok(())
    }
}
