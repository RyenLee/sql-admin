use axum::{
    Router, middleware as axum_middleware,
    routing::{delete, get, post},
};
use tower_http::{cors::CorsLayer, services::ServeDir, trace::TraceLayer};

use crate::handlers;
use crate::middleware;
use crate::state::AppState;

pub fn create_router_with_frontend(
    app_state: AppState,
    frontend_dist: Option<String>,
) -> Router {
    let api_routes = Router::new()
        .route(
            "/api/connections",
            post(handlers::connections::create_connection)
                .get(handlers::connections::list_connections),
        )
        .route(
            "/api/connections/{id}",
            get(handlers::connections::get_connection)
                .put(handlers::connections::update_connection)
                .delete(handlers::connections::delete_connection),
        )
        .route(
            "/api/connections/{id}/test",
            post(handlers::connections::test_connection),
        )
        .route(
            "/api/connections/test",
            post(handlers::connections::test_connection_request),
        )
        .route("/api/query", post(handlers::query::execute_query))
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
        .route(
            "/api/connections/{id}/redb/tables",
            get(handlers::redb_browser::list_tables),
        )
        .route(
            "/api/connections/{id}/redb/query",
            post(handlers::redb_browser::query_keys),
        )
        .route(
            "/api/connections/{id}/redb/edit",
            post(handlers::redb_browser::edit_key),
        )
        .route(
            "/api/connections/{id}/redb/batch-delete",
            post(handlers::redb_browser::batch_delete),
        )
        .route("/health", get(health_check))
        .layer(axum_middleware::from_fn(middleware::logging))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(app_state);

    // If frontend_dist is configured, serve static files and add SPA fallback
    if let Some(dist_path) = frontend_dist {
        let dist = std::path::Path::new(&dist_path);
        if dist.exists() {
            tracing::info!(
                module = "router",
                event = "frontend_static_enabled",
                path = %dist_path,
                "Serving frontend static files"
            );
            let index_html = dist.join("index.html");
            Router::new()
                .merge(api_routes)
                .fallback_service(
                    ServeDir::new(&dist_path)
                        .fallback(axum::routing::get_service(
                            tower_http::services::ServeFile::new(index_html),
                        )),
                )
        } else {
            tracing::warn!(
                module = "router",
                event = "frontend_dist_not_found",
                path = %dist_path,
                "FRONTEND_DIST directory not found, serving API only"
            );
            api_routes
        }
    } else {
        api_routes
    }
}

async fn health_check() -> &'static str {
    "OK"
}