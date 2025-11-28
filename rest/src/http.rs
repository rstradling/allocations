/*!
    Module `http` exposes an HTTP server that handles HTTP requests to the application. Its
    implementation is opaque to module consumers.
*/

mod api;
mod employee_handlers;
use std::sync::Arc;

use crate::http::employee_handlers::{
    create_employee, delete_employee, get_employee, get_employees, update_employee,
};
use anyhow::Context;
use axum::Router;
use axum::routing::{delete, get, post, put};
use repository::employee_repo::EmployeeRepo;
use repository::postgres_db::PostgresDb;
use serde_json::json;
use sqlx::PgPool;
use tokio::net;

#[derive(Debug, Clone)]
/// The global application state shared between all request handlers.
pub struct AppState<Repo: EmployeeRepo> {
    employee_repo: Arc<Repo>,
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

        let employee_repo = <PostgresDb as EmployeeRepo>::new(pool);
        // Construct dependencies to inject into handlers.
        let state = AppState {
            employee_repo: Arc::new(employee_repo),
        };

        let router = axum::Router::new()
            .route("/health", get(health_check))
            .nest("/api/employees", employee_routes())
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

fn employee_routes<RR: EmployeeRepo>() -> Router<AppState<RR>> {
    Router::new()
        .route("/", post(create_employee::<RR>))
        .route("/{id}", get(get_employee::<RR>))
        .route("/{id}", put(update_employee::<RR>))
        .route("/{id}", delete(delete_employee::<RR>))
        .route("/", get(get_employees::<RR>))
}
