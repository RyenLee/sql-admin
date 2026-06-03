use std::collections::HashMap;
use sql_admin_domain::connection::value_objects::DatabaseType;
use sql_admin_domain::shared::infrastructure_error::InfrastructureError;
use tokio::sync::RwLock;
use tracing;

use super::factory::{DynPool, PoolFactory};

pub struct CachedPoolManager<F: PoolFactory> {
    factory: F,
    pools: RwLock<HashMap<String, DynPool>>,
}

impl<F: PoolFactory> CachedPoolManager<F> {
    pub fn new(factory: F) -> Self {
        Self {
            factory,
            pools: RwLock::new(HashMap::new()),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn get_or_create(
        &self,
        connection_id: &str,
        database_type: &DatabaseType,
        host: &str,
        port: u16,
        database: &str,
        username: &str,
        password: &str,
    ) -> Result<DynPool, InfrastructureError> {
        {
            let mut pools = self.pools.write().await;
            if let Some(pool) = pools.get(connection_id) {
                return Ok(pool.clone());
            }

            tracing::info!(
                module = "pool",
                event = "create_pool",
                connection_id = %connection_id,
                database_type = %database_type,
                "Creating new connection pool"
            );

            let pool = self
                .factory
                .create_pool(database_type, host, port, database, username, password)
                .await?;

            pools.insert(connection_id.to_string(), pool.clone());

            tracing::info!(
                module = "pool",
                event = "pool_created",
                connection_id = %connection_id,
                "Connection pool created and cached"
            );

            Ok(pool)
        }
    }

    pub async fn invalidate(&self, connection_id: &str) {
        let mut pools = self.pools.write().await;
        pools.remove(connection_id);
        tracing::info!(
            module = "pool",
            event = "pool_invalidated",
            connection_id = %connection_id,
            "Connection pool cache invalidated"
        );
    }

    pub async fn get_cached_or_err(&self, connection_id: &str) -> Result<DynPool, InfrastructureError> {
        let pools = self.pools.read().await;
        pools.get(connection_id).cloned().ok_or_else(|| {
            InfrastructureError::PoolCreationFailed(format!(
                "No cached pool found for connection: {}",
                connection_id
            ))
        })
    }
}