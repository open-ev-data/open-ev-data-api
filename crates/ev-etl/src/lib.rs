use anyhow::{Context, Result};
use tracing::{info, warn};

pub mod cli;
pub mod error;
pub mod ingest;
pub mod merge;
pub mod output;
pub mod validate;

use cli::Cli;

pub fn run_validation(cli: &Cli) -> Result<()> {
    info!("Running validation-only mode");

    let raw_vehicles = ingest::load_dataset(&cli.input)?;
    info!("Loaded {} vehicle files", raw_vehicles.len());

    let merged_vehicles = merge::merge_all(&raw_vehicles)?;
    info!("Merged into {} vehicles", merged_vehicles.len());

    let results = validate::validate_all(&merged_vehicles);
    let (valid, invalid): (Vec<_>, Vec<_>) = results.into_iter().partition(Result::is_ok);

    info!(
        "Validation complete: {} valid, {} invalid",
        valid.len(),
        invalid.len()
    );

    if !invalid.is_empty() {
        for result in &invalid {
            if let Err(e) = result {
                tracing::error!("{}", e);
            }
        }
        anyhow::bail!("{} vehicles failed validation", invalid.len());
    }

    Ok(())
}

pub fn run_pipeline(cli: &Cli) -> Result<()> {
    let start_time = std::time::Instant::now();

    info!("Input directory: {:?}", cli.input);
    info!("Output directory: {:?}", cli.output);
    info!("Formats: {:?}", cli.formats);

    std::fs::create_dir_all(&cli.output).context("Failed to create output directory")?;

    let raw_vehicles = ingest::load_dataset(&cli.input)?;
    info!("Loaded {} raw vehicle files", raw_vehicles.len());

    let merged_vehicles = merge::merge_all(&raw_vehicles)?;
    info!("Merged into {} canonical vehicles", merged_vehicles.len());

    let validation_results = validate::validate_all(&merged_vehicles);
    let valid_vehicles: Vec<_> = validation_results
        .into_iter()
        .zip(merged_vehicles.into_iter())
        .filter_map(|(result, vehicle)| {
            if result.is_ok() {
                Some(vehicle)
            } else {
                warn!("Skipping invalid vehicle: {:?}", result.err());
                None
            }
        })
        .collect();

    info!("{} vehicles passed validation", valid_vehicles.len());

    for format in &cli.formats {
        match format.as_str() {
            "json" => {
                let path = cli.output.join("vehicles.json");
                output::json::generate(&valid_vehicles, &path)?;
                info!("Generated: {:?}", path);
            }
            "sqlite" => {
                let path = cli.output.join("vehicles.db");
                output::sqlite::generate(&valid_vehicles, &path)?;
                info!("Generated: {:?}", path);
            }
            "postgresql" => {
                let path = cli.output.join("vehicles.sql");
                output::postgresql::generate(&valid_vehicles, &path)?;
                info!("Generated: {:?}", path);
            }
            "csv" => {
                let path = cli.output.join("vehicles.csv");
                output::csv::generate(&valid_vehicles, &path)?;
                info!("Generated: {:?}", path);
            }
            "xml" => {
                let path = cli.output.join("vehicles.xml");
                output::xml::generate(&valid_vehicles, &path)?;
                info!("Generated: {:?}", path);
            }
            _ => warn!("Unknown format: {}", format),
        }
    }

    let stats = output::statistics::generate(&valid_vehicles, start_time.elapsed());
    let stats_path = cli.output.join("statistics.json");
    std::fs::write(&stats_path, serde_json::to_string_pretty(&stats)?)?;
    info!("Generated: {:?}", stats_path);

    info!(
        "Pipeline complete in {:.2}s",
        start_time.elapsed().as_secs_f64()
    );

    Ok(())
}
