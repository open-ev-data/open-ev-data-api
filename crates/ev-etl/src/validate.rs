use anyhow::Result;
use ev_core::{Validate, Vehicle};
use serde_json::Value;
use std::path::Path;

pub fn validate_all(vehicles: &[Vehicle]) -> Vec<Result<()>> {
    vehicles
        .iter()
        .map(|vehicle| {
            vehicle
                .validate()
                .map_err(|e| anyhow::anyhow!("Validation failed for {}: {}", vehicle.id(), e))
        })
        .collect()
}

#[allow(dead_code)]
pub fn validate_vehicle(vehicle: &Vehicle) -> Result<()> {
    vehicle
        .validate()
        .map_err(|e| anyhow::anyhow!("Validation failed: {}", e))
}

#[allow(dead_code)]
pub fn validate_with_json_schema(vehicle_json: &Value, schema: &Value) -> Result<Vec<String>> {
    let compiled =
        jsonschema::validator_for(schema).map_err(|e| anyhow::anyhow!("Invalid schema: {}", e))?;

    let errors: Vec<String> = compiled
        .iter_errors(vehicle_json)
        .map(|error| format!("{} at {}", error, error.instance_path))
        .collect();

    Ok(errors)
}

#[allow(dead_code)]
pub fn load_schema(schema_path: &Path) -> Result<Value> {
    let content = std::fs::read_to_string(schema_path)
        .map_err(|e| anyhow::anyhow!("Failed to read schema file: {}", e))?;
    serde_json::from_str(&content)
        .map_err(|e| anyhow::anyhow!("Failed to parse schema JSON: {}", e))
}

#[derive(Debug, Clone, serde::Serialize)]
#[allow(dead_code)]
pub struct ValidationReport {
    pub total_vehicles: usize,
    pub valid_count: usize,
    pub error_count: usize,
    pub errors: Vec<ValidationError>,
}

#[derive(Debug, Clone, serde::Serialize)]
#[allow(dead_code)]
pub struct ValidationError {
    pub vehicle_id: String,
    pub error_type: String,
    pub message: String,
    pub path: String,
}

#[allow(dead_code)]
impl ValidationReport {
    pub fn new() -> Self {
        Self {
            total_vehicles: 0,
            valid_count: 0,
            error_count: 0,
            errors: Vec::new(),
        }
    }

    pub fn add_error(&mut self, vehicle_id: &str, error_type: &str, message: &str, path: &str) {
        self.error_count += 1;
        self.errors.push(ValidationError {
            vehicle_id: vehicle_id.to_string(),
            error_type: error_type.to_string(),
            message: message.to_string(),
            path: path.to_string(),
        });
    }

    pub fn save_to_file(&self, path: &Path) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }
}

impl Default for ValidationReport {
    fn default() -> Self {
        Self::new()
    }
}
