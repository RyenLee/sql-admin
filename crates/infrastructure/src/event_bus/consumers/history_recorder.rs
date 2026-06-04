use std::sync::Arc;
use sql_admin_domain::history::aggregate::QueryHistory as DomainQueryHistory;
use sql_admin_domain::history::repository::QueryHistoryRepository;
use sql_admin_domain::shared::event::DomainEvent;
use tokio::sync::broadcast;
use tracing::{info, warn, error};

pub async fn start_history_recorder_consumer(
    mut rx: broadcast::Receiver<DomainEvent>,
    history_repo: Arc<dyn QueryHistoryRepository>,
) {
    loop {
        match rx.recv().await {
            Ok(event) => match event {
                DomainEvent::QueryExecuted {
                    connection_id,
                    connection_name,
                    query_text,
                    execution_time_ms,
                    success,
                    ..
                } => {
                    let (history, _events) = DomainQueryHistory::create(
                        connection_id.clone(),
                        connection_name,
                        query_text,
                        Some(execution_time_ms),
                        None,
                        success,
                        None,
                    );
                    match history_repo.save(&history).await {
                        Ok(_) => {
                            info!(
                                module = "event_consumer::history_recorder",
                                connection_id = %connection_id,
                                execution_time_ms = execution_time_ms,
                                success = success,
                                "Query history persisted via event"
                            );
                        }
                        Err(e) => {
                            warn!(
                                module = "event_consumer::history_recorder",
                                connection_id = %connection_id,
                                error = %e,
                                "Failed to persist query history"
                            );
                        }
                    }
                }
                DomainEvent::HistoryCleared { .. } => {
                    match history_repo.delete_all().await {
                        Ok(_) => {
                            info!(
                                module = "event_consumer::history_recorder",
                                "History cleared via event"
                            );
                        }
                        Err(e) => {
                            warn!(
                                module = "event_consumer::history_recorder",
                                error = %e,
                                "Failed to clear history"
                            );
                        }
                    }
                }
                _ => {}
            },
            Err(broadcast::error::RecvError::Lagged(n)) => {
                error!(
                    module = "event_consumer::history_recorder",
                    skipped = n,
                    "History recorder consumer lagged behind, some query history records may be lost"
                );
            }
            Err(broadcast::error::RecvError::Closed) => {
                info!(
                    module = "event_consumer::history_recorder",
                    "Event bus closed, stopping consumer"
                );
                break;
            }
        }
    }
}