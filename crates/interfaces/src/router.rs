use axum::{
    Router, middleware as axum_middleware,
    routing::{delete, get, post},
};
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::handlers;
use crate::middleware;
use crate::state::AppState;

pub fn create_router(app_state: AppState) -> Router {
    Router::new()
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
        .with_state(app_state)
}

async fn health_check() -> &'static str {
    "OK"
}