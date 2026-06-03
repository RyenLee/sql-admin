use std::sync::Arc;
use sql_admin_domain::shared::event::DomainEvent;
use tokio::sync::broadcast;
use tracing::{info, warn};

use crate::pool::factory::CachedDomainPoolFactory;

#[async_trait::async_trait]
pub trait PoolCacheInvalidator: Send + Sync {
    async fn invalidate(&self, connection_id: &str);
}

pub struct NoopPoolInvalidator;

#[async_trait::async_trait]
impl PoolCacheInvalidator for NoopPoolInvalidator {
    async fn invalidate(&self, connection_id: &str) {
        info!(
            module = "event_consumer::pool_invalidation",
            connection_id = %connection_id,
            "Pool invalidation requested (noop)"
        );
    }
}

#[async_trait::async_trait]
impl PoolCacheInvalidator for CachedDomainPoolFactory {
    async fn invalidate(&self, connection_id: &str) {
        CachedDomainPoolFactory::invalidate(self, connection_id);
    }
}

pub async fn start_pool_invalidation_consumer(
    mut rx: broadcast::Receiver<DomainEvent>,
    invalidator: Arc<dyn PoolCacheInvalidator>,
) {
    loop {
        match rx.recv().await {
            Ok(event) => match event {
                DomainEvent::ConnectionUpdated { ref connection_id, .. }
                | DomainEvent::ConnectionDeleted { ref connection_id, .. } => {
                    info!(
                        module = "event_consumer::pool_invalidation",
                        event_type = ?event,
                        connection_id = %connection_id,
                        "Invalidating connection pool cache"
                    );
                    invalidator.invalidate(connection_id).await;
                }
                _ => {}
            },
            Err(broadcast::error::RecvError::Lagged(n)) => {
                warn!(
                    module = "event_consumer::pool_invalidation",
                    skipped = n,
                    "Pool invalidation consumer lagged behind"
                );
            }
            Err(broadcast::error::RecvError::Closed) => {
                info!(
                    module = "event_consumer::pool_invalidation",
                    "Event bus closed, stopping consumer"
                );
                break;
            }
        }
    }
}