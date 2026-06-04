use std::sync::Arc;
use sql_admin_application::app_state::AppState;
use sql_admin_application::connection::handler::ConnectionHandler;
use sql_admin_application::connection_pool_service::ConnectionPoolService;
use sql_admin_application::query::handler::QueryHandler;
use sql_admin_application::data_edit::handler::DataEditHandler;
use sql_admin_application::history::handler::HistoryHandler;
use sql_admin_application::import::handler::ImportHandler;
use sql_admin_application::redb::handler::RedbHandler;
use sql_admin_infrastructure::crypto::aes_gcm::AesGcmEncryptionService;
use sql_admin_infrastructure::persistence::sqlite_connection_repo::SqliteConnectionRepository;
use sql_admin_infrastructure::persistence::sqlite_history_repo::SqliteQueryHistoryRepository;
use sql_admin_infrastructure::pool::factory::CachedDomainPoolFactory;
use sql_admin_infrastructure::event_bus::in_memory::InMemoryEventBus;
use sql_admin_infrastructure::event_bus::consumers::{history_recorder, pool_invalidation};
use tauri::Manager;

mod commands;
mod state;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            // Resolve app data directory for SQLite database
            let data_dir = app
                .path()
                .app_local_data_dir()
                .map_err(|e| format!("Failed to resolve app data directory: {}", e))?;
            std::fs::create_dir_all(&data_dir)
                .map_err(|e| format!("Failed to create data directory: {}", e))?;

            let db_path = data_dir.join("sql_admin.sqlite3");
            let db_url = format!("sqlite:{}?mode=rwc", db_path.display());

            tracing::info!(
                path = %db_path.display(),
                "SQLite database path"
            );

            // Create tokio runtime (leaked to keep alive for the app lifetime)
            let rt = Box::leak(Box::new(
                tokio::runtime::Runtime::new().map_err(|e| format!("Failed to create tokio runtime: {}", e))?,
            ));

            let app_state = rt.block_on(async {
                let pool = sqlx::SqlitePool::connect(&db_url)
                    .await
                    .map_err(|e| format!("Failed to connect to SQLite: {}", e))?;
                sqlx::migrate!("../infrastructure/src/persistence/migrations")
                    .run(&pool)
                    .await
                    .map_err(|e| format!("Failed to run migrations: {}", e))?;

                let crypto: Arc<dyn sql_admin_domain::shared::crypto::EncryptionService> =
                    Arc::new(AesGcmEncryptionService::new().unwrap_or_else(|e| {
                        tracing::warn!(
                            module = "desktop_main",
                            error = %e,
                            "ENCRYPTION_KEY not set, using default key (development only)"
                        );
                        #[allow(deprecated)]
                        AesGcmEncryptionService::with_default_key()
                    }));

                let conn_repo: Arc<
                    dyn sql_admin_domain::connection::repository::ConnectionRepository,
                > = Arc::new(SqliteConnectionRepository::new(pool.clone()));
                let history_repo: Arc<
                    dyn sql_admin_domain::history::repository::QueryHistoryRepository,
                > = Arc::new(SqliteQueryHistoryRepository::new(pool.clone()));

                let cached_pool_factory = Arc::new(CachedDomainPoolFactory::new());
                let pool_factory: Arc<dyn sql_admin_domain::shared::pool::PoolFactory> =
                    cached_pool_factory.clone();
                let redb_executor: Arc<dyn sql_admin_domain::shared::pool::RedbExecutor> =
                    cached_pool_factory.clone();

                let event_bus = InMemoryEventBus::new();
                let pool_consumer_rx = event_bus.subscribe();
                let history_consumer_rx = event_bus.subscribe();
                let event_bus: Arc<dyn sql_admin_domain::shared::event::EventBus> =
                    Arc::new(event_bus);

                tokio::spawn(pool_invalidation::start_pool_invalidation_consumer(
                    pool_consumer_rx,
                    cached_pool_factory.clone(),
                ));
                tokio::spawn(history_recorder::start_history_recorder_consumer(
                    history_consumer_rx,
                    history_repo.clone(),
                ));

                let connection_pool_service = ConnectionPoolService::new(
                    conn_repo.clone(),
                    pool_factory.clone(),
                    crypto.clone(),
                );

                Ok::<AppState, String>(AppState::new(
                    ConnectionHandler::new(
                        conn_repo.clone(),
                        crypto.clone(),
                        event_bus.clone(),
                        pool_factory.clone(),
                        connection_pool_service,
                    ),
                    QueryHandler::new(
                        conn_repo.clone(),
                        pool_factory.clone(),
                        crypto.clone(),
                        event_bus.clone(),
                    ),
                    HistoryHandler::new(history_repo),
                    DataEditHandler::new(conn_repo.clone(), pool_factory.clone(), crypto.clone(), event_bus),
                    ImportHandler::new(conn_repo.clone(), pool_factory.clone(), crypto.clone()),
                    RedbHandler::new(conn_repo, pool_factory, redb_executor, crypto),
                ))
            })?;

            app.manage(app_state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::connections::list_connections,
            commands::connections::create_connection,
            commands::connections::get_connection,
            commands::connections::update_connection,
            commands::connections::delete_connection,
            commands::connections::test_connection,
            commands::connections::test_connection_request,
            commands::query::execute_query,
            commands::query::get_table_data,
            commands::schema::get_schema,
            commands::schema::get_table_def,
            commands::history::get_query_history,
            commands::history::save_query_history,
            commands::history::clear_query_history,
            commands::history::delete_query_history_item,
            commands::data_edit::edit_row,
            commands::data_edit::delete_row,
            commands::data_edit::insert_row,
            commands::import::import_sql,
            commands::redb_browser::list_redb_tables,
            commands::redb_browser::query_redb_keys,
            commands::redb_browser::edit_redb_key,
            commands::redb_browser::batch_delete_redb_keys,
        ])
        .run(tauri::generate_context!())
        .map_err(|e| tracing::error!("Application error: {}", e)).unwrap_or(());
}

fn main() {
    run();
}
