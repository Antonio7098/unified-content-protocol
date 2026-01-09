//! REST API server for the benchmark UI.

pub mod handlers;
pub mod routes;
pub mod state;

pub use state::AppState;

use axum::{
    http::Method,
    routing::{delete, get, post},
    Router,
};
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};

/// Create the API router
pub fn create_router(state: Arc<RwLock<AppState>>) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::DELETE,
            Method::PUT,
            Method::PATCH,
        ])
        .allow_origin(Any)
        .allow_headers(Any);

    Router::new()
        // Registry endpoints
        .route("/api/registry", get(handlers::get_registry))
        .route("/api/registry/categories", get(handlers::get_categories))
        .route("/api/registry/categories/:id", get(handlers::get_category))
        .route(
            "/api/registry/categories/:id/tests",
            get(handlers::get_category_tests),
        )
        .route("/api/tests", get(handlers::list_tests))
        .route("/api/tests/:id", get(handlers::get_test))
        // Provider endpoints
        .route("/api/providers", get(handlers::get_providers))
        .route(
            "/api/providers/available",
            get(handlers::get_available_providers),
        )
        // Suite endpoints
        .route("/api/suites", get(handlers::list_suites))
        .route("/api/suites", post(handlers::create_suite))
        .route("/api/suites/:id", get(handlers::get_suite))
        .route("/api/suites/:id", delete(handlers::delete_suite))
        .route(
            "/api/suites/:id",
            axum::routing::put(handlers::update_suite),
        )
        // Run endpoints (LLM benchmarks)
        .route("/api/runs", get(handlers::list_runs))
        .route("/api/runs", post(handlers::start_run))
        .route("/api/runs/:id", get(handlers::get_run))
        .route("/api/runs/:id/cancel", post(handlers::cancel_run))
        .route("/api/runs/:id/results", get(handlers::get_run_results))
        .route(
            "/api/runs/:id/results/:test_id",
            get(handlers::get_test_result),
        )
        // Core benchmark endpoints
        .route("/api/core/benchmarks", get(handlers::list_core_benchmarks))
        .route("/api/core/inputs", get(handlers::list_core_inputs))
        .route("/api/core/inputs/:id", get(handlers::get_core_input))
        .route("/api/core/categories", get(handlers::list_core_categories))
        .route(
            "/api/core/categories/:id/inputs",
            get(handlers::get_core_category_inputs),
        )
        .route("/api/core/runs", get(handlers::list_core_runs))
        .route("/api/core/runs", post(handlers::start_core_benchmark))
        .route("/api/core/runs/:id", get(handlers::get_core_run))
        .route("/api/core/runs/:id", delete(handlers::delete_core_run))
        .route(
            "/api/core/runs/:id/results/:test_id",
            get(handlers::get_core_test_result),
        )
        .route("/api/core/history", get(handlers::list_core_history))
        // WebSocket for real-time updates
        .route("/api/ws", get(handlers::websocket_handler))
        // Document preview
        .route("/api/documents", get(handlers::list_documents))
        .route("/api/documents/:id", get(handlers::get_document))
        .route("/api/document", get(handlers::get_test_document))
        // Playground endpoints
        .route(
            "/api/playground/documents",
            get(handlers::list_playground_documents),
        )
        .route(
            "/api/playground/documents/:id",
            get(handlers::get_playground_document),
        )
        .route(
            "/api/playground/chat",
            axum::routing::post(handlers::playground_chat),
        )
        .layer(cors)
        .with_state(state)
}

/// Start the API server
pub async fn start_server(port: u16) -> anyhow::Result<()> {
    let state = Arc::new(RwLock::new(AppState::new()));
    let app = create_router(state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    tracing::info!("API server listening on port {}", port);

    axum::serve(listener, app).await?;
    Ok(())
}
