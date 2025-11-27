/*!
    Module `http` exposes an HTTP server that handles HTTP requests to the application. Its
    implementation is opaque to module consumers.
*/

mod api;
mod roster_handlers;
use std::sync::Arc;

use crate::http::roster_handlers::{create_roster_item, get_roster_item, update_roster_item};
use anyhow::Context;
use axum::Router;
use axum::routing::{get, post, put};
use repository::postgres_db::PostgresDb;
use repository::roster_repo::RosterRepo;
use serde_json::json;
use sqlx::PgPool;
use tokio::net;

#[derive(Debug, Clone)]
/// The global application state shared between all request handlers.
pub struct AppState<Repo: RosterRepo> {
    roster_repo: Arc<Repo>,
}

/// Configuration for the HTTP server.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpServerConfig<'a> {
    pub port: &'a str,
}

/// The application's HTTP server. The underlying HTTP package is opaque to module consumers.
pub struct HttpServer {
    router: axum::Router,
    listener: net::TcpListener,
}

async fn health_check() -> axum::Json<serde_json::Value> {
    axum::Json(json!({
        "status": "healthy",
        "service": "your-service-name",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

impl HttpServer {
    /// Returns a new HTTP server bound to the port specified in `config`.
    pub async fn new(pool: PgPool, config: HttpServerConfig<'_>) -> anyhow::Result<Self> {
        let trace_layer = tower_http::trace::TraceLayer::new_for_http().make_span_with(
            |request: &axum::extract::Request<_>| {
                let uri = request.uri().to_string();
                tracing::info_span!("http_request", method = ?request.method(), uri)
            },
        );

        let roster_repo = <PostgresDb as RosterRepo>::new(pool);
        // Construct dependencies to inject into handlers.
        let state = AppState {
            roster_repo: Arc::new(roster_repo),
        };

        let router = axum::Router::new()
            .route("/health", get(health_check))
            .nest("/api/roster", roster_routes())
            .layer(trace_layer)
            .with_state(state);

        let listener = net::TcpListener::bind(format!("0.0.0.0:{}", config.port))
            .await
            .with_context(|| format!("failed to listen on {}", config.port))?;

        Ok(Self { router, listener })
    }

    /// Runs the HTTP server.
    pub async fn run(self) -> anyhow::Result<()> {
        tracing::debug!("listening on {}", self.listener.local_addr().unwrap());
        axum::serve(self.listener, self.router)
            .await
            .context("received error from running server")?;
        Ok(())
    }
}

fn roster_routes<RR: RosterRepo>() -> Router<AppState<RR>> {
    Router::new()
        .route("/", post(create_roster_item::<RR>))
        .route("/{id}", get(get_roster_item::<RR>))
        .route("/{id}", put(update_roster_item::<RR>))
    /*.route("/roster/{id}", delete(delete_roster_item::<RR>))
    .route("/roster", get(get_all_roster_items::<RR>))*/
}
