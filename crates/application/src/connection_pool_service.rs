use std::sync::Arc;
use sql_admin_domain::connection::repository::ConnectionRepository;
use sql_admin_domain::connection::value_objects::DatabaseType;
use sql_admin_domain::shared::application_error::ApplicationError;
use sql_admin_domain::shared::crypto::EncryptionService;
use sql_admin_domain::shared::pool::{PoolFactory, QueryExecutor};

pub struct ConnectionPoolService {
    conn_repo: Arc<dyn ConnectionRepository>,
    pool_factory: Arc<dyn PoolFactory>,
    crypto: Arc<dyn EncryptionService>,
}

impl ConnectionPoolService {
    pub fn new(
        conn_repo: Arc<dyn ConnectionRepository>,
        pool_factory: Arc<dyn PoolFactory>,
        crypto: Arc<dyn EncryptionService>,
    ) -> Self {
        Self {
            conn_repo,
            pool_factory,
            crypto,
        }
    }

    pub async fn get_executor(
        &self,
        connection_id: &str,
    ) -> Result<Arc<dyn QueryExecutor>, ApplicationError> {
        let conn = self
            .conn_repo
            .find_by_id(connection_id)
            .await?
            .ok_or_else(|| {
                ApplicationError::NotFound(format!("Connection not found: {}", connection_id))
            })?;

        let password = conn
            .encrypted_password()
            .decrypt(self.crypto.as_ref())?;

        let executor = self
            .pool_factory
            .create_pool(
                connection_id,
                conn.database_type(),
                conn.host(),
                conn.port(),
                conn.database(),
                conn.username(),
                &password,
            )
            .await?;

        Ok(executor)
    }

    pub async fn get_executor_with_type(
        &self,
        connection_id: &str,
    ) -> Result<(Arc<dyn QueryExecutor>, DatabaseType), ApplicationError> {
        let conn = self
            .conn_repo
            .find_by_id(connection_id)
            .await?
            .ok_or_else(|| {
                ApplicationError::NotFound(format!("Connection not found: {}", connection_id))
            })?;

        let db_type = conn.database_type().clone();
        let password = conn
            .encrypted_password()
            .decrypt(self.crypto.as_ref())?;

        let executor = self
            .pool_factory
            .create_pool(
                connection_id,
                conn.database_type(),
                conn.host(),
                conn.port(),
                conn.database(),
                conn.username(),
                &password,
            )
            .await?;

        Ok((executor, db_type))
    }
}
