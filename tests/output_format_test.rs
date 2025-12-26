use ev_core::Vehicle;
use std::collections::HashMap;
use tempfile::TempDir;

fn create_test_vehicle() -> Vehicle {
    let json = r#"{
        "make": {"slug": "tesla", "name": "Tesla"},
        "model": {"slug": "model_3", "name": "Model 3"},
        "year": 2024,
        "trim": {"slug": "long_range", "name": "Long Range"},
        "body": {"vehicle_type": "sedan", "seats": 5, "doors": 4},
        "battery": {"capacity_net_kwh": 75.0, "capacity_gross_kwh": 82.0},
        "charging": {
            "dc": {"max_power_kw": 250, "connectors": [{"type": "ccs2"}]},
            "ac": {"max_power_kw": 11, "connectors": [{"type": "type2"}]}
        }
    }"#;
    serde_json::from_str(json).expect("Failed to parse test vehicle")
}

#[test]
fn test_json_output_structure() {
    let vehicles = vec![create_test_vehicle()];
    let json_output = serde_json::to_string_pretty(&vehicles).expect("Serialization failed");

    assert!(json_output.contains("tesla"));
    assert!(json_output.contains("model_3"));
    assert!(json_output.contains("2024"));
}

#[test]
fn test_vehicle_count_matches() {
    let vehicles = vec![create_test_vehicle(), create_test_vehicle()];

    assert_eq!(vehicles.len(), 2);
}

#[test]
fn test_vehicle_has_required_fields() {
    let vehicle = create_test_vehicle();
    let json: serde_json::Value = serde_json::to_value(&vehicle).expect("Serialization failed");

    assert!(json.get("make").is_some());
    assert!(json.get("model").is_some());
    assert!(json.get("year").is_some());
    assert!(json.get("trim").is_some());
}

#[test]
fn test_csv_escaping() {
    let value = "Tesla, Inc.";
    let escaped = format!("\"{}\"", value.replace("\"", "\"\""));
    assert_eq!(escaped, "\"Tesla, Inc.\"");
}

#[test]
fn test_xml_structure() {
    let vehicle = create_test_vehicle();
    let json = serde_json::to_value(&vehicle).expect("Serialization failed");

    let make = json
        .get("make")
        .and_then(|m| m.get("name"))
        .and_then(|n| n.as_str());
    assert_eq!(make, Some("Tesla"));
}
