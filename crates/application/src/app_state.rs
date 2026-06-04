use std::sync::Arc;
use crate::connection::handler::ConnectionHandler;
use crate::data_edit::handler::DataEditHandler;
use crate::history::handler::HistoryHandler;
use crate::import::handler::ImportHandler;
use crate::query::handler::QueryHandler;
use crate::redb::handler::RedbHandler;

#[derive(Clone)]
pub struct AppState {
    pub connection_handler: Arc<ConnectionHandler>,
    pub query_handler: Arc<QueryHandler>,
    pub history_handler: Arc<HistoryHandler>,
    pub data_edit_handler: Arc<DataEditHandler>,
    pub import_handler: Arc<ImportHandler>,
    pub redb_handler: Arc<RedbHandler>,
}

impl AppState {
    pub fn new(
        connection_handler: ConnectionHandler,
        query_handler: QueryHandler,
        history_handler: HistoryHandler,
        data_edit_handler: DataEditHandler,
        import_handler: ImportHandler,
        redb_handler: RedbHandler,
    ) -> Self {
        Self {
            connection_handler: Arc::new(connection_handler),
            query_handler: Arc::new(query_handler),
            history_handler: Arc::new(history_handler),
            data_edit_handler: Arc::new(data_edit_handler),
            import_handler: Arc::new(import_handler),
            redb_handler: Arc::new(redb_handler),
        }
    }
}
