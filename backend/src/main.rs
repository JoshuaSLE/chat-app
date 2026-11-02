use dotenvy::dotenv;
use tracing_subscriber::EnvFilter;

fn main() {
    dotenv().ok();

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::fmt().with_env_filter(filter).init();

    tracing::info!("backend starting");
}
