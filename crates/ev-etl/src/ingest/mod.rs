mod parser;
mod reader;

use std::path::Path;

use anyhow::{Context, Result};
use serde_json::Value;

pub use reader::{FileType, VehicleFile};

pub fn load_dataset(input_dir: &Path) -> Result<Vec<VehicleFile>> {
    reader::scan_directory(input_dir).context("Failed to scan dataset directory")
}

#[allow(dead_code)]
pub fn parse_json_file(path: &Path) -> Result<Value> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {:?}", path))?;

    serde_json::from_str(&content).with_context(|| format!("Failed to parse JSON: {:?}", path))
}
