use anyhow::Context;
use dotenvy::dotenv;
use sqlx::{migrate, postgres::PgPoolOptions};
use std::{env, time::Duration};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::fmt().with_env_filter(filter).init();

    tracing::info!("backend starting");

    let database_url = env::var("DATABASE_URL").context("DATABASE_URL must be set")?;

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&database_url)
        .await
        .context("failed to connect to database")?;

    migrate!().run(&pool).await.context("migrations failed")?;

    tracing::info!("database ready");

    Ok(())
}
