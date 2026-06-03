use std::sync::Arc;
use sql_admin_application::connection::handler::ConnectionHandler;
use sql_admin_application::connection_pool_service::ConnectionPoolService;
use sql_admin_application::data_edit::handler::DataEditHandler;
use sql_admin_application::history::handler::HistoryHandler;
use sql_admin_application::import::handler::ImportHandler;
use sql_admin_application::query::handler::QueryHandler;
use sql_admin_application::redb::handler::RedbHandler;
use sql_admin_infrastructure::crypto::aes_gcm::AesGcmEncryptionService;
use sql_admin_infrastructure::persistence::sqlite_connection_repo::SqliteConnectionRepository;
use sql_admin_infrastructure::persistence::sqlite_history_repo::SqliteQueryHistoryRepository;
use sql_admin_infrastructure::pool::factory::CachedDomainPoolFactory;
use sql_admin_infrastructure::event_bus::in_memory::InMemoryEventBus;
use sql_admin_infrastructure::event_bus::consumers::{history_recorder, pool_invalidation};

mod config;
mod error;
mod handlers;
mod logging;
mod middleware;
mod router;
mod state;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let config = config::AppConfig::from_env()?;
    let _log_guard = logging::init_logging(&config);

    tracing::info!(
        module = "main",
        event = "service_initializing",
        server_addr = %config.server_addr,
        environment = ?config.environment,
        log_level = %config.log_level,
        "Service initialization started"
    );

    let pool = sqlx::SqlitePool::connect(&config.database_url).await?;
    tracing::info!(
        module = "main",
        event = "database_connected",
        "Database connection established"
    );

    sqlx::migrate!("../infrastructure/src/persistence/migrations").run(&pool).await?;
    tracing::info!(
        module = "main",
        event = "database_migrated",
        "Database migrations applied"
    );

    let crypto: Arc<dyn sql_admin_domain::shared::crypto::EncryptionService> =
        Arc::new(match AesGcmEncryptionService::new() {
            Ok(service) => {
                tracing::info!(
                    module = "main",
                    event = "encryption_initialized",
                    "Using ENCRYPTION_KEY from environment"
                );
                service
            }
            Err(e) => {
                tracing::warn!(
                    module = "main",
                    event = "encryption_fallback",
                    error = %e,
                    "ENCRYPTION_KEY not set, using default key (for development only)"
                );
                AesGcmEncryptionService::with_default_key()
            }
        });

    let conn_repo: Arc<dyn sql_admin_domain::connection::repository::ConnectionRepository> =
        Arc::new(SqliteConnectionRepository::new(pool.clone()));
    let history_repo: Arc<dyn sql_admin_domain::history::repository::QueryHistoryRepository> =
        Arc::new(SqliteQueryHistoryRepository::new(pool.clone()));
    let cached_pool_factory = Arc::new(CachedDomainPoolFactory::new());
    let pool_factory: Arc<dyn sql_admin_domain::shared::pool::PoolFactory> =
        cached_pool_factory.clone();
    let redb_executor: Arc<dyn sql_admin_domain::shared::pool::RedbExecutor> =
        cached_pool_factory.clone();

    let event_bus = InMemoryEventBus::new();
    let pool_consumer_rx = event_bus.subscribe();
    let history_consumer_rx = event_bus.subscribe();
    let event_bus: Arc<dyn sql_admin_domain::shared::event::EventBus> = Arc::new(event_bus);

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

    let connection_handler = ConnectionHandler::new(conn_repo.clone(), crypto.clone(), event_bus.clone(), pool_factory.clone(), connection_pool_service);
    let query_handler = QueryHandler::new(conn_repo.clone(), pool_factory.clone(), crypto.clone(), event_bus.clone());
    let history_handler = HistoryHandler::new(history_repo.clone());
    let data_edit_handler = DataEditHandler::new(conn_repo.clone(), pool_factory.clone(), crypto.clone(), event_bus.clone());
    let import_handler = ImportHandler::new(conn_repo.clone(), pool_factory.clone(), crypto.clone());
    let redb_handler = RedbHandler::new(
        conn_repo.clone(),
        pool_factory.clone(),
        redb_executor.clone(),
        crypto.clone(),
    );

    let app_state = state::AppState::new(
        connection_handler,
        query_handler,
        history_handler,
        data_edit_handler,
        import_handler,
        redb_handler,
    );

    let router = router::create_router(app_state);

    // Create listener with SO_REUSEADDR to handle TIME_WAIT state on Windows
    let addr: std::net::SocketAddr = config.server_addr.parse()
        .map_err(|e| anyhow::anyhow!("Invalid server address '{}': {}", config.server_addr, e))?;
    
    let socket = match addr {
        std::net::SocketAddr::V4(_) => tokio::net::TcpSocket::new_v4()?,
        std::net::SocketAddr::V6(_) => tokio::net::TcpSocket::new_v6()?,
    };
    
    socket.set_reuseaddr(true)?;
    socket.bind(addr)?;
    
    let listener = socket.listen(1024)?;

    tracing::info!(
        module = "main",
        event = "service_started",
        server_addr = %config.server_addr,
        "Server running and ready to accept connections"
    );

    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    tracing::info!(
        module = "main",
        event = "service_stopped",
        "Server shutdown completed"
    );

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(windows)]
    let terminate = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(not(windows))]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = ctrl_c => {
            tracing::warn!(
                module = "main",
                event = "shutdown_signal",
                signal = "ctrl_c",
                "Received Ctrl+C signal, initiating graceful shutdown"
            );
        },
        _ = terminate => {
            tracing::warn!(
                module = "main",
                event = "shutdown_signal",
                signal = "terminate",
                "Received terminate signal, initiating graceful shutdown"
            );
        },
    }
}