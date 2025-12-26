use std::path::Path;

use anyhow::{Context, Result};
use chrono::Utc;
use ev_core::Vehicle;
use serde::Serialize;
use sha2::{Digest, Sha256};

#[derive(Serialize)]
pub struct CanonicalOutput {
    pub schema_version: String,
    pub generated_at: String,
    pub vehicle_count: usize,
    pub vehicles: Vec<Vehicle>,
    pub metadata: OutputMetadata,
}

#[derive(Serialize)]
pub struct OutputMetadata {
    pub etl_version: String,
    pub processing_time_ms: u64,
}

pub fn generate(vehicles: &[Vehicle], output_path: &Path) -> Result<()> {
    let start = std::time::Instant::now();

    let output = CanonicalOutput {
        schema_version: "1.0.0".to_string(),
        generated_at: Utc::now().to_rfc3339(),
        vehicle_count: vehicles.len(),
        vehicles: vehicles.to_vec(),
        metadata: OutputMetadata {
            etl_version: env!("CARGO_PKG_VERSION").to_string(),
            processing_time_ms: start.elapsed().as_millis() as u64,
        },
    };

    let json = serde_json::to_string_pretty(&output)
        .context("Failed to serialize vehicles to JSON")?;

    std::fs::write(output_path, &json)
        .with_context(|| format!("Failed to write JSON to {:?}", output_path))?;

    let mut hasher = Sha256::new();
    hasher.update(json.as_bytes());
    let hash = hex::encode(hasher.finalize());

    let checksum_path = output_path.with_extension("json.sha256");
    std::fs::write(&checksum_path, format!("{}  {}\n", hash, output_path.file_name().unwrap().to_string_lossy()))
        .with_context(|| format!("Failed to write checksum to {:?}", checksum_path))?;

    Ok(())
}
