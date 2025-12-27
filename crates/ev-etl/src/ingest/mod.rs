pub mod parser;
pub mod reader;

use std::path::Path;

use anyhow::{Context, Result};

pub use parser::parse_json_file;
pub use reader::{FileType, VehicleFile};

pub fn load_dataset(input_dir: &Path) -> Result<Vec<VehicleFile>> {
    reader::scan_directory(input_dir).context("Failed to scan dataset directory")
}
