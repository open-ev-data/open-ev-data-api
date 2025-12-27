use anyhow::{Context, Result};
use clap::Parser;
use ev_etl::{cli::Cli, run_pipeline, run_validation};
use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let level = if cli.verbose {
        Level::DEBUG
    } else {
        Level::INFO
    };
    let subscriber = FmtSubscriber::builder()
        .with_max_level(level)
        .with_target(false)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .context("Failed to set tracing subscriber")?;

    info!("OpenEV Data ETL Pipeline v{}", env!("CARGO_PKG_VERSION"));

    if cli.validate_only {
        return run_validation(&cli);
    }

    run_pipeline(&cli)
}
