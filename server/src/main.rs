use axum::{Router, routing::get};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/", get(|| async { "Server is running" }))
        .route("/health", get(|| async { "OK" }))
        .nest_service("/static", ServeDir::new("static"));

    let addr = "0.0.0.0:3000";
    tracing::info!("Starting server on {}", addr);

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
