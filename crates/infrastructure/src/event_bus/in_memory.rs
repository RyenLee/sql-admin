use async_trait::async_trait;
use sql_admin_domain::shared::event::{DomainEvent, EventBus};
use sql_admin_domain::shared::infrastructure_error::InfrastructureError;
use tokio::sync::broadcast;

pub struct InMemoryEventBus {
    sender: broadcast::Sender<DomainEvent>,
}

impl InMemoryEventBus {
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(256);
        Self { sender }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<DomainEvent> {
        self.sender.subscribe()
    }
}

#[async_trait]
impl EventBus for InMemoryEventBus {
    async fn publish(&self, event: DomainEvent) -> Result<(), InfrastructureError> {
        self.sender.send(event).map_err(|e| {
            InfrastructureError::EventBusError(format!("Failed to publish event: {}", e))
        })?;
        Ok(())
    }
}

impl Default for InMemoryEventBus {
    fn default() -> Self {
        Self::new()
    }
}