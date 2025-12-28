use ev_core::{
    Battery, ChargePort, Charging, ConnectorType, Drivetrain, PortKind, Powertrain, Range,
    RangeCycle, RangeRated, SlugName, Source, SourceType, Vehicle, VehicleType,
};
use ev_etl::validate::{
    ValidationReport, load_schema, validate_all, validate_vehicle, validate_with_json_schema,
};
use tempfile::NamedTempFile;

fn create_valid_vehicle() -> Vehicle {
    Vehicle {
        schema_url: None,
        schema_version: "1.0.0".to_string(),
        make: SlugName {
            slug: "tesla".into(),
            name: "Tesla".into(),
        },
        model: SlugName {
            slug: "model_y".into(),
            name: "Model Y".into(),
        },
        year: 2024,
        trim: SlugName {
            slug: "base".into(),
            name: "Base".into(),
        },
        vehicle_type: VehicleType::Suv,
        powertrain: Powertrain {
            drivetrain: Drivetrain::Rwd,
            system_power_kw: Some(220.0),
            system_torque_nm: None,
            motors: None,
            transmission: None,
        },
        battery: Battery {
            pack_capacity_kwh_net: Some(60.0),
            pack_capacity_kwh_gross: Some(65.0),
            ..Default::default()
        },
        charge_ports: vec![ChargePort {
            kind: PortKind::Combo,
            connector: ConnectorType::Ccs2,
            location: None,
            covers: None,
            light: None,
            motorized: None,
            notes: None,
        }],
        charging: Charging::default(),
        range: Range {
            rated: vec![RangeRated {
                cycle: RangeCycle::Wltp,
                range_km: 450.0,
                notes: None,
            }],
            real_world: None,
        },
        sources: vec![Source {
            source_type: SourceType::Oem,
            title: "Official".to_string(),
            url: "https://example.com".to_string(),
            accessed_at: "2024-01-01".to_string(),
            publisher: None,
            license: None,
            notes: None,
        }],
        unique_code: None,
        variant: None,
        markets: None,
        availability: None,
        body: None,
        dimensions: None,
        weights: None,
        capacity: None,
        v2x: None,
        efficiency: None,
        performance: None,
        wheels_tires: None,
        pricing: None,
        software: None,
        links: None,
        images: None,
        metadata: None,
    }
}

fn create_invalid_vehicle() -> Vehicle {
    Vehicle {
        schema_url: None,
        schema_version: "1.0.0".to_string(),
        make: SlugName {
            slug: "tesla".into(),
            name: "Tesla".into(),
        },
        model: SlugName {
            slug: "model_y".into(),
            name: "Model Y".into(),
        },
        year: 2024,
        trim: SlugName {
            slug: "base".into(),
            name: "Base".into(),
        },
        vehicle_type: VehicleType::Suv,
        powertrain: Powertrain {
            drivetrain: Drivetrain::Rwd,
            system_power_kw: None,
            system_torque_nm: None,
            motors: None,
            transmission: None,
        },
        battery: Battery {
            pack_capacity_kwh_net: Some(60.0),
            ..Default::default()
        },
        charge_ports: vec![],
        charging: Charging::default(),
        range: Range {
            rated: vec![],
            real_world: None,
        },
        sources: vec![],
        unique_code: None,
        variant: None,
        markets: None,
        availability: None,
        body: None,
        dimensions: None,
        weights: None,
        capacity: None,
        v2x: None,
        efficiency: None,
        performance: None,
        wheels_tires: None,
        pricing: None,
        software: None,
        links: None,
        images: None,
        metadata: None,
    }
}

#[test]
fn test_validate_vehicle_success() {
    let vehicle = create_valid_vehicle();
    let result = validate_vehicle(&vehicle);
    assert!(result.is_ok());
}

#[test]
fn test_validate_vehicle_fail() {
    let vehicle = create_invalid_vehicle();
    let result = validate_vehicle(&vehicle);
    assert!(result.is_err());
}

#[test]
fn test_validate_all_all_valid() {
    let v1 = create_valid_vehicle();
    let v2 = create_valid_vehicle();

    let results = validate_all(&[v1, v2]);
    assert_eq!(results.len(), 2);
    assert!(results.iter().all(|r| r.is_ok()));
}

#[test]
fn test_validate_all_all_invalid() {
    let v1 = create_invalid_vehicle();
    let v2 = create_invalid_vehicle();

    let results = validate_all(&[v1, v2]);
    assert_eq!(results.len(), 2);
    assert!(results.iter().all(|r| r.is_err()));
}

#[test]
fn test_validate_all_mixed() {
    let v1 = create_valid_vehicle();
    let v2 = create_invalid_vehicle();

    let results = validate_all(&[v1, v2]);
    assert_eq!(results.len(), 2);
    assert!(results[0].is_ok());
    assert!(results[1].is_err());
}

#[test]
fn test_validate_all_empty() {
    let results = validate_all(&[]);
    assert!(results.is_empty());
}

#[test]
fn test_validation_report_new() {
    let report = ValidationReport::new();

    assert_eq!(report.total_vehicles, 0);
    assert_eq!(report.valid_count, 0);
    assert_eq!(report.error_count, 0);
    assert!(report.errors.is_empty());
}

#[test]
fn test_validation_report_default() {
    let report = ValidationReport::default();

    assert_eq!(report.total_vehicles, 0);
    assert_eq!(report.valid_count, 0);
    assert_eq!(report.error_count, 0);
    assert!(report.errors.is_empty());
}

#[test]
fn test_validation_report_add_error() {
    let mut report = ValidationReport::new();

    report.add_error(
        "vehicle_1",
        "missing_field",
        "Field is required",
        "/sources",
    );

    assert_eq!(report.error_count, 1);
    assert_eq!(report.errors.len(), 1);
    assert_eq!(report.errors[0].vehicle_id, "vehicle_1");
    assert_eq!(report.errors[0].error_type, "missing_field");
    assert_eq!(report.errors[0].message, "Field is required");
    assert_eq!(report.errors[0].path, "/sources");
}

#[test]
fn test_validation_report_add_multiple_errors() {
    let mut report = ValidationReport::new();

    report.add_error(
        "vehicle_1",
        "missing_field",
        "Field is required",
        "/sources",
    );
    report.add_error(
        "vehicle_1",
        "invalid_value",
        "Value out of range",
        "/battery/capacity",
    );
    report.add_error("vehicle_2", "missing_field", "Field is required", "/range");

    assert_eq!(report.error_count, 3);
    assert_eq!(report.errors.len(), 3);
}

#[test]
fn test_validation_report_save_to_file() {
    let mut report = ValidationReport::new();
    report.total_vehicles = 10;
    report.valid_count = 8;
    report.add_error(
        "vehicle_1",
        "missing_field",
        "Field is required",
        "/sources",
    );
    report.add_error("vehicle_2", "invalid_value", "Invalid year", "/year");

    let file = NamedTempFile::new().expect("Failed to create temp file");
    let path = file.path();

    report.save_to_file(path).expect("Failed to save report");

    let content = std::fs::read_to_string(path).expect("Failed to read file");
    let parsed: serde_json::Value = serde_json::from_str(&content).expect("Invalid JSON");

    assert_eq!(parsed["total_vehicles"], 10);
    assert_eq!(parsed["valid_count"], 8);
    assert_eq!(parsed["error_count"], 2);
    assert!(parsed["errors"].is_array());
    assert_eq!(parsed["errors"].as_array().unwrap().len(), 2);
}

#[test]
fn test_validation_report_serialization() {
    let mut report = ValidationReport::new();
    report.total_vehicles = 5;
    report.valid_count = 4;
    report.add_error("vehicle_1", "test_error", "Test message", "/path");

    let json = serde_json::to_value(&report).expect("Serialization failed");

    assert_eq!(json["total_vehicles"], 5);
    assert_eq!(json["valid_count"], 4);
    assert_eq!(json["error_count"], 1);
    assert_eq!(json["errors"][0]["vehicle_id"], "vehicle_1");
}

#[test]
fn test_validate_with_json_schema_valid() {
    let schema = serde_json::json!({
        "type": "object",
        "properties": {
            "name": {"type": "string"},
            "year": {"type": "integer"}
        },
        "required": ["name"]
    });

    let valid_json = serde_json::json!({
        "name": "Tesla",
        "year": 2024
    });

    let errors = validate_with_json_schema(&valid_json, &schema).expect("Validation failed");
    assert!(errors.is_empty());
}

#[test]
fn test_validate_with_json_schema_invalid() {
    let schema = serde_json::json!({
        "type": "object",
        "properties": {
            "name": {"type": "string"},
            "year": {"type": "integer"}
        },
        "required": ["name", "year"]
    });

    let invalid_json = serde_json::json!({
        "name": "Tesla"
    });

    let errors = validate_with_json_schema(&invalid_json, &schema).expect("Validation failed");
    assert!(!errors.is_empty());
}

#[test]
fn test_validate_with_json_schema_type_mismatch() {
    let schema = serde_json::json!({
        "type": "object",
        "properties": {
            "year": {"type": "integer"}
        }
    });

    let invalid_json = serde_json::json!({
        "year": "not a number"
    });

    let errors = validate_with_json_schema(&invalid_json, &schema).expect("Validation failed");
    assert!(!errors.is_empty());
}

#[test]
fn test_validate_with_json_schema_nested_object() {
    let schema = serde_json::json!({
        "type": "object",
        "properties": {
            "vehicle": {
                "type": "object",
                "properties": {
                    "make": {"type": "string"}
                },
                "required": ["make"]
            }
        }
    });

    let valid_json = serde_json::json!({
        "vehicle": {
            "make": "Tesla"
        }
    });

    let errors = validate_with_json_schema(&valid_json, &schema).expect("Validation failed");
    assert!(errors.is_empty());
}

#[test]
fn test_load_schema_from_file() {
    let schema_content = r#"{
        "type": "object",
        "properties": {
            "name": {"type": "string"}
        }
    }"#;

    let file = NamedTempFile::new().expect("Failed to create temp file");
    let path = file.path();
    std::fs::write(path, schema_content).expect("Failed to write schema");

    let schema = load_schema(path).expect("Failed to load schema");

    assert_eq!(schema["type"], "object");
    assert!(schema["properties"]["name"].is_object());
}

#[test]
fn test_load_schema_file_not_found() {
    let path = std::path::Path::new("/nonexistent/path/schema.json");
    let result = load_schema(path);
    assert!(result.is_err());
}

#[test]
fn test_load_schema_invalid_json() {
    let file = NamedTempFile::new().expect("Failed to create temp file");
    let path = file.path();
    std::fs::write(path, "not valid json {").expect("Failed to write content");

    let result = load_schema(path);
    assert!(result.is_err());
}

#[test]
fn test_validation_error_clone() {
    let mut report = ValidationReport::new();
    report.add_error("v1", "error_type", "message", "/path");

    let error = report.errors[0].clone();
    assert_eq!(error.vehicle_id, "v1");
    assert_eq!(error.error_type, "error_type");
}

#[test]
fn test_validation_report_clone() {
    let mut report = ValidationReport::new();
    report.total_vehicles = 5;
    report.add_error("v1", "error", "msg", "/path");

    let cloned = report.clone();
    assert_eq!(cloned.total_vehicles, 5);
    assert_eq!(cloned.errors.len(), 1);
}

#[test]
fn test_validation_report_debug() {
    let report = ValidationReport::new();
    let debug_str = format!("{:?}", report);
    assert!(debug_str.contains("ValidationReport"));
}

#[test]
fn test_save_to_file_failure() {
    let report = ValidationReport::new();
    let temp_dir = NamedTempFile::new().expect("Failed to create temp file");
    let path = temp_dir.path().parent().unwrap();

    let result = report.save_to_file(path);
    assert!(result.is_err());
}

#[test]
fn test_validate_with_json_schema_invalid_schema() {
    let invalid_schema = serde_json::json!({
        "type": "invalid_type",
        "properties": {}
    });

    let valid_json = serde_json::json!({});

    let result = validate_with_json_schema(&valid_json, &invalid_schema);
    assert!(result.is_err());
}
