mod templates;

use axum::{
    Router,
    routing::{get, post},
};
use tokio::net::TcpListener;
use tower_http::{services::ServeDir, trace::TraceLayer, compression::CompressionLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                #[cfg(debug_assertions)]
                {
                    "debug".into()
                }
                #[cfg(not(debug_assertions))]
                {
                    "info".into()
                }
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/", get(templates::index))
        .route("/health", get(|| async { "OK" }))
        .route("/clicked", post(templates::clicked))
        .nest_service(
            "/static",
            ServeDir::new("../static").append_index_html_on_directories(false),
        )
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http());

    let addr = "0.0.0.0:3000";
    tracing::info!("Starting server on {}", addr);

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
