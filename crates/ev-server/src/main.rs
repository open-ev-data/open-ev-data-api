use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::{Context, Result};
use tokio::net::TcpListener;
use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;

use ev_server::{api, config::Config, db::Database};

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::from_env()?;

    let level = match config.log_level.as_str() {
        "debug" => Level::DEBUG,
        "warn" => Level::WARN,
        "error" => Level::ERROR,
        _ => Level::INFO,
    };

    let subscriber = FmtSubscriber::builder()
        .with_max_level(level)
        .with_target(false)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .context("Failed to set tracing subscriber")?;

    info!("OpenEV Data API Server v{}", env!("CARGO_PKG_VERSION"));

    let database = Database::new(&config.database_url).context("Failed to connect to database")?;
    let db = Arc::new(database);

    info!("Connected to database: {}", config.database_url);

    let app = api::create_router(db, &config);

    let addr: SocketAddr = format!("{}:{}", config.host, config.port)
        .parse()
        .context("Invalid address")?;

    info!("Starting server on http://{}", addr);

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
