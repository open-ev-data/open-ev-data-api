//! JSON parsing utilities for vehicle data files.

use std::path::Path;

use anyhow::{Context, Result};
use serde_json::Value;

/// Parse a JSON file and return its contents as a serde_json Value.
#[allow(dead_code)]
pub fn parse_json_file(path: &Path) -> Result<Value> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {:?}", path))?;

    serde_json::from_str(&content).with_context(|| format!("Failed to parse JSON: {:?}", path))
}

/// Validate that a JSON value is an object (not array, null, etc.)
#[allow(dead_code)]
pub fn validate_json_object(value: &Value, path: &Path) -> Result<()> {
    if !value.is_object() {
        anyhow::bail!(
            "Expected JSON object in {:?}, got {}",
            path,
            json_type_name(value)
        );
    }
    Ok(())
}

/// Get a human-readable type name for a JSON value.
#[allow(dead_code)]
fn json_type_name(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_json_type_name() {
        assert_eq!(json_type_name(&json!(null)), "null");
        assert_eq!(json_type_name(&json!(true)), "boolean");
        assert_eq!(json_type_name(&json!(42)), "number");
        assert_eq!(json_type_name(&json!("test")), "string");
        assert_eq!(json_type_name(&json!([])), "array");
        assert_eq!(json_type_name(&json!({})), "object");
    }

    #[test]
    fn test_validate_json_object() {
        let obj = json!({"key": "value"});
        let path = Path::new("test.json");
        assert!(validate_json_object(&obj, path).is_ok());

        let arr = json!([1, 2, 3]);
        assert!(validate_json_object(&arr, path).is_err());
    }
}
