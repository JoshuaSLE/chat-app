use anyhow::Context;
use axum::Router;
use dotenvy::dotenv;
use sqlx::{migrate, postgres::PgPoolOptions};
use std::{env, time::Duration};
use tokio::{net::TcpListener, signal};
use tracing_subscriber::EnvFilter;

mod routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::fmt().with_env_filter(filter).init();

    tracing::info!("backend starting");

    let database_url = env::var("DATABASE_URL").context("DATABASE_URL must be set")?;
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .context("PORT must be a valid port number")?;

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&database_url)
        .await
        .context("failed to connect to database")?;

    migrate!().run(&pool).await.context("migrations failed")?;

    tracing::info!("database ready");

    let app = Router::new().nest("/api/v1", routes::router());

    let listener = TcpListener::bind((host.as_str(), port))
        .await
        .context("failed to bind TCP listener")?;

    tracing::info!("server listening on {}", listener.local_addr()?);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("server error")?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("shutdown signal received");
}
