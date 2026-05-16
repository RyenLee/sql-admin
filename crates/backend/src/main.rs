mod config;
mod crypto;
mod db;
mod error;
mod handlers;
mod id_generator;
mod logging;
mod middleware;
mod state;

use axum::{
    Router, middleware as axum_middleware,
    routing::{delete, get, post},
};
use std::collections::HashMap;
use std::sync::Arc;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    // 从环境变量中读取配置
    let config = config::AppConfig::from_env()?;
    // 初始化日志
    let _log_guard = logging::init_logging(&config);

    tracing::info!(
        module = "main",
        event = "service_initializing",
        server_addr = %config.server_addr,
        environment = ?config.environment,
        log_level = %config.log_level,
        "Service initialization started"
    );

    // 连接数据库
    let pool = sqlx::SqlitePool::connect(&config.database_url).await?;
    tracing::info!(
        module = "main",
        event = "database_connected",
        database_url = "******",
        "Database connection established"
    );

    sqlx::migrate!("./migrations").run(&pool).await?;
    tracing::info!(
        module = "main",
        event = "database_migrated",
        "Database migrations applied"
    );

    // 初始化应用状态
    let app_state = Arc::new(state::AppState {
        db_pool: pool,
        config: config.clone(),
        connection_pools: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
    });

    // 初始化路由
    let router = Router::new()
        .route(
            "/api/connections",
            post(handlers::connections::create_connection)
                .get(handlers::connections::list_connections),
        )
        .route(
            "/api/connections/test",
            post(handlers::connections::test_connection_request),
        )
        .route(
            "/api/connections/{id}",
            get(handlers::connections::get_connection)
                .put(handlers::connections::update_connection)
                .delete(handlers::connections::delete_connection),
        )
        .route("/api/query", post(handlers::query::execute_query))
        .route(
            "/api/connections/{id}/test",
            post(handlers::connections::test_connection),
        )
        .route(
            "/api/connections/{id}/schema",
            get(handlers::schema::get_schema),
        )
        .route(
            "/api/connections/{id}/tables/{table}/def",
            get(handlers::schema::get_table_def),
        )
        .route(
            "/api/connections/{id}/tables/{table}/data",
            get(handlers::query::get_table_data),
        )
        .route("/api/history", get(handlers::history::get_query_history))
        .route("/api/history", post(handlers::history::save_query_history))
        .route(
            "/api/history",
            delete(handlers::history::delete_query_history),
        )
        .route(
            "/api/history/{id}",
            delete(handlers::history::delete_query_history_item),
        )
        .route(
            "/api/connections/{id}/edit-row",
            post(handlers::data_edit::edit_row),
        )
        .route(
            "/api/connections/{id}/delete-row",
            post(handlers::data_edit::delete_row),
        )
        .route(
            "/api/connections/{id}/insert-row",
            post(handlers::data_edit::insert_row),
        )
        .route(
            "/api/connections/{id}/import",
            post(handlers::import::import_sql),
        )
        .route("/health", get(health_check))
        .layer(axum_middleware::from_fn(middleware::logging))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(&config.server_addr).await?;
    // 服务器启动
    tracing::info!(
        module = "main",
        event = "service_started",
        server_addr = %config.server_addr,
        "Server running and ready to accept connections"
    );

    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    // 服务器关闭
    tracing::info!(
        module = "main",
        event = "service_stopped",
        "Server shutdown completed"
    );

    Ok(())
}

async fn health_check() -> &'static str {
    "OK"
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
