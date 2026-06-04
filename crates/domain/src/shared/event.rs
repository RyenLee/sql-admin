use chrono::{DateTime, Utc};
use super::infrastructure_error::InfrastructureError;

#[derive(Clone, Debug)]
pub enum DomainEvent {
    ConnectionCreated {
        connection_id: String,
        database_type: String,
        timestamp: DateTime<Utc>,
    },
    ConnectionUpdated {
        connection_id: String,
        changed_fields: Vec<String>,
        timestamp: DateTime<Utc>,
    },
    ConnectionDeleted {
        connection_id: String,
        timestamp: DateTime<Utc>,
    },
    QueryExecuted {
        connection_id: String,
        connection_name: String,
        query_text: String,
        query_hash: String,
        execution_time_ms: u64,
        success: bool,
        timestamp: DateTime<Utc>,
    },
    HistoryCleared {
        timestamp: DateTime<Utc>,
    },
    RowEdited {
        connection_id: String,
        table_name: String,
        column: String,
        timestamp: DateTime<Utc>,
    },
    RedbKeyModified {
        connection_id: String,
        table: String,
        key: String,
        operation: String,
        timestamp: DateTime<Utc>,
    },
}

#[async_trait::async_trait]
pub trait EventBus: Send + Sync {
    async fn publish(&self, event: DomainEvent) -> Result<(), InfrastructureError>;
}