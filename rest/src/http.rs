/*!
    Module `http` exposes an HTTP server that handles HTTP requests to the application. Its
    implementation is opaque to module consumers.
*/

mod api;
use std::sync::Arc;

use anyhow::Context;
use axum::routing::get;
use serde_json::json;
use tokio::net;

trait DummyService {}
#[derive(Debug, Clone)]
/// The global application state shared between all request handlers.
pub struct AppState<DS: DummyService> {
    dummy_service: Arc<DS>,
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
    pub async fn new(config: HttpServerConfig<'_>) -> anyhow::Result<Self> {
        let trace_layer = tower_http::trace::TraceLayer::new_for_http().make_span_with(
            |request: &axum::extract::Request<_>| {
                let uri = request.uri().to_string();
                tracing::info_span!("http_request", method = ?request.method(), uri)
            },
        );

        // Construct dependencies to inject into handlers.
        //let state = AppState {
        //    pet_service: Arc::new(svc),
        //};

        let router = axum::Router::new()
            .route("/health", get(health_check))
            //.nest("/api", api_routes())
            .layer(trace_layer);
        //   .with_state(state);

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

/*fn api_routes<DS: DummyService>() -> Router<AppState<DS>> {
    Router::new()
        .route("/pets", post(create_pet::<PS>))
        .route("/pets", post(create_pet::<PS>))
        .route("/pets/{id}", get(get_pet::<PS>))
        .route("/pets/{id}", put(update_pet::<PS>))
}*/
