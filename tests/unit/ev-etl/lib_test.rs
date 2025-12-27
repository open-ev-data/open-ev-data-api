use ev_etl::cli::Cli;
use ev_etl::{run_pipeline, run_validation};
use serde_json::json;
use std::path::PathBuf;
use tempfile::TempDir;

fn create_valid_test_dataset() -> TempDir {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let base = temp_dir.path();

    std::fs::create_dir_all(base.join("tesla/model_3/2024")).expect("Failed to create dirs");

    let vehicle = json!({
        "schema_version": "1.0.0",
        "make": {"slug": "tesla", "name": "Tesla"},
        "model": {"slug": "model_3", "name": "Model 3"},
        "year": 2024,
        "trim": {"slug": "base", "name": "Base"},
        "vehicle_type": "passenger_car",
        "powertrain": {"drivetrain": "rwd", "system_power_kw": 208.0},
        "battery": {"pack_capacity_kwh_net": 60.0},
        "charge_ports": [{"kind": "combo", "connector": "ccs2"}],
        "charging": {"ac": {"max_power_kw": 11.0}},
        "range": {"rated": [{"cycle": "wltp", "range_km": 513.0}]},
        "sources": [{"type": "oem", "title": "Tesla", "url": "https://tesla.com", "accessed_at": "2024-01-01"}]
    });

    std::fs::write(
        base.join("tesla/model_3/2024/tesla_model_3.json"),
        serde_json::to_string_pretty(&vehicle).unwrap(),
    )
    .expect("Failed to write");

    temp_dir
}

fn create_cli(input: PathBuf, output: PathBuf, formats: Vec<String>, validate_only: bool) -> Cli {
    Cli {
        input,
        output,
        formats,
        validate_only,
        verbose: false,
    }
}

#[test]
fn test_run_validation_success() {
    let input_dir = create_valid_test_dataset();
    let cli = create_cli(
        input_dir.path().to_path_buf(),
        PathBuf::from("/tmp/unused"),
        vec!["json".to_string()],
        true,
    );

    let result = run_validation(&cli);
    assert!(result.is_ok());
}

#[test]
fn test_run_validation_empty_dataset() {
    let input_dir = TempDir::new().expect("Failed to create temp dir");
    let cli = create_cli(
        input_dir.path().to_path_buf(),
        PathBuf::from("/tmp/unused"),
        vec!["json".to_string()],
        true,
    );

    let result = run_validation(&cli);
    assert!(result.is_ok());
}

#[test]
fn test_run_pipeline_json_format() {
    let input_dir = create_valid_test_dataset();
    let output_dir = TempDir::new().expect("Failed to create output dir");

    let cli = create_cli(
        input_dir.path().to_path_buf(),
        output_dir.path().to_path_buf(),
        vec!["json".to_string()],
        false,
    );

    let result = run_pipeline(&cli);
    assert!(result.is_ok());

    assert!(output_dir.path().join("vehicles.json").exists());
    assert!(output_dir.path().join("statistics.json").exists());
}

#[test]
fn test_run_pipeline_csv_format() {
    let input_dir = create_valid_test_dataset();
    let output_dir = TempDir::new().expect("Failed to create output dir");

    let cli = create_cli(
        input_dir.path().to_path_buf(),
        output_dir.path().to_path_buf(),
        vec!["csv".to_string()],
        false,
    );

    let result = run_pipeline(&cli);
    assert!(result.is_ok());
    assert!(output_dir.path().join("vehicles.csv").exists());
}

#[test]
fn test_run_pipeline_xml_format() {
    let input_dir = create_valid_test_dataset();
    let output_dir = TempDir::new().expect("Failed to create output dir");

    let cli = create_cli(
        input_dir.path().to_path_buf(),
        output_dir.path().to_path_buf(),
        vec!["xml".to_string()],
        false,
    );

    let result = run_pipeline(&cli);
    assert!(result.is_ok());
    assert!(output_dir.path().join("vehicles.xml").exists());
}

#[test]
fn test_run_pipeline_sqlite_format() {
    let input_dir = create_valid_test_dataset();
    let output_dir = TempDir::new().expect("Failed to create output dir");

    let cli = create_cli(
        input_dir.path().to_path_buf(),
        output_dir.path().to_path_buf(),
        vec!["sqlite".to_string()],
        false,
    );

    let result = run_pipeline(&cli);
    assert!(result.is_ok());
    assert!(output_dir.path().join("vehicles.db").exists());
}

#[test]
fn test_run_pipeline_postgresql_format() {
    let input_dir = create_valid_test_dataset();
    let output_dir = TempDir::new().expect("Failed to create output dir");

    let cli = create_cli(
        input_dir.path().to_path_buf(),
        output_dir.path().to_path_buf(),
        vec!["postgresql".to_string()],
        false,
    );

    let result = run_pipeline(&cli);
    assert!(result.is_ok());
    assert!(output_dir.path().join("vehicles.sql").exists());
}

#[test]
fn test_run_pipeline_all_formats() {
    let input_dir = create_valid_test_dataset();
    let output_dir = TempDir::new().expect("Failed to create output dir");

    let cli = create_cli(
        input_dir.path().to_path_buf(),
        output_dir.path().to_path_buf(),
        vec![
            "json".to_string(),
            "csv".to_string(),
            "xml".to_string(),
            "sqlite".to_string(),
            "postgresql".to_string(),
        ],
        false,
    );

    let result = run_pipeline(&cli);
    assert!(result.is_ok());

    assert!(output_dir.path().join("vehicles.json").exists());
    assert!(output_dir.path().join("vehicles.csv").exists());
    assert!(output_dir.path().join("vehicles.xml").exists());
    assert!(output_dir.path().join("vehicles.db").exists());
    assert!(output_dir.path().join("vehicles.sql").exists());
    assert!(output_dir.path().join("statistics.json").exists());
}

#[test]
fn test_run_pipeline_unknown_format() {
    let input_dir = create_valid_test_dataset();
    let output_dir = TempDir::new().expect("Failed to create output dir");

    let cli = create_cli(
        input_dir.path().to_path_buf(),
        output_dir.path().to_path_buf(),
        vec!["unknown_format".to_string()],
        false,
    );

    let result = run_pipeline(&cli);
    assert!(result.is_ok());
}

#[test]
fn test_run_pipeline_creates_output_dir() {
    let input_dir = create_valid_test_dataset();
    let output_base = TempDir::new().expect("Failed to create base dir");
    let output_path = output_base.path().join("new_output_dir");

    let cli = create_cli(
        input_dir.path().to_path_buf(),
        output_path.clone(),
        vec!["json".to_string()],
        false,
    );

    let result = run_pipeline(&cli);
    assert!(result.is_ok());
    assert!(output_path.exists());
}

#[test]
fn test_run_pipeline_empty_dataset() {
    let input_dir = TempDir::new().expect("Failed to create temp dir");
    let output_dir = TempDir::new().expect("Failed to create output dir");

    let cli = create_cli(
        input_dir.path().to_path_buf(),
        output_dir.path().to_path_buf(),
        vec!["json".to_string()],
        false,
    );

    let result = run_pipeline(&cli);
    assert!(result.is_ok());
}

fn create_invalid_test_dataset() -> TempDir {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let base = temp_dir.path();

    std::fs::create_dir_all(base.join("tesla/model_3/2024")).expect("Failed to create dirs");

    // Invalid vehicle: missing battery capacity, charge ports, range, sources
    let vehicle = json!({
        "schema_version": "1.0.0",
        "make": {"slug": "tesla", "name": "Tesla"},
        "model": {"slug": "model_3", "name": "Model 3"},
        "year": 2024,
        "trim": {"slug": "base", "name": "Base"},
        "vehicle_type": "passenger_car",
        "powertrain": {"drivetrain": "rwd"},
        "battery": {},
        "charge_ports": [],
        "charging": {},
        "range": {"rated": []},
        "sources": []
    });

    std::fs::write(
        base.join("tesla/model_3/2024/tesla_model_3.json"),
        serde_json::to_string_pretty(&vehicle).unwrap(),
    )
    .expect("Failed to write");

    temp_dir
}

#[test]
fn test_run_validation_with_invalid_vehicle() {
    let input_dir = create_invalid_test_dataset();
    let cli = create_cli(
        input_dir.path().to_path_buf(),
        PathBuf::from("/tmp/unused"),
        vec!["json".to_string()],
        true,
    );

    let result = run_validation(&cli);
    // Should fail because the vehicle is invalid
    assert!(result.is_err());
}

#[test]
fn test_run_pipeline_filters_invalid_vehicles() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let base = temp_dir.path();

    std::fs::create_dir_all(base.join("tesla/model_3/2024")).expect("Failed to create dirs");
    std::fs::create_dir_all(base.join("byd/dolphin/2024")).expect("Failed to create dirs");

    // Valid vehicle
    let valid_vehicle = json!({
        "schema_version": "1.0.0",
        "make": {"slug": "tesla", "name": "Tesla"},
        "model": {"slug": "model_3", "name": "Model 3"},
        "year": 2024,
        "trim": {"slug": "base", "name": "Base"},
        "vehicle_type": "passenger_car",
        "powertrain": {"drivetrain": "rwd", "system_power_kw": 208.0},
        "battery": {"pack_capacity_kwh_net": 60.0},
        "charge_ports": [{"kind": "combo", "connector": "ccs2"}],
        "charging": {"ac": {"max_power_kw": 11.0}},
        "range": {"rated": [{"cycle": "wltp", "range_km": 513.0}]},
        "sources": [{"type": "oem", "title": "Tesla", "url": "https://tesla.com", "accessed_at": "2024-01-01"}]
    });

    // Invalid vehicle: missing required fields
    let invalid_vehicle = json!({
        "schema_version": "1.0.0",
        "make": {"slug": "byd", "name": "BYD"},
        "model": {"slug": "dolphin", "name": "Dolphin"},
        "year": 2024,
        "trim": {"slug": "base", "name": "Base"},
        "vehicle_type": "passenger_car",
        "powertrain": {"drivetrain": "rwd"},
        "battery": {},
        "charge_ports": [],
        "charging": {},
        "range": {"rated": []},
        "sources": []
    });

    std::fs::write(
        base.join("tesla/model_3/2024/tesla_model_3.json"),
        serde_json::to_string_pretty(&valid_vehicle).unwrap(),
    )
    .expect("Failed to write");

    std::fs::write(
        base.join("byd/dolphin/2024/byd_dolphin.json"),
        serde_json::to_string_pretty(&invalid_vehicle).unwrap(),
    )
    .expect("Failed to write");

    let output_dir = TempDir::new().expect("Failed to create output dir");

    let cli = create_cli(
        base.to_path_buf(),
        output_dir.path().to_path_buf(),
        vec!["json".to_string()],
        false,
    );

    // Pipeline should succeed but skip invalid vehicle
    let result = run_pipeline(&cli);
    assert!(result.is_ok());

    // Verify output contains only valid vehicle
    let output_json = std::fs::read_to_string(output_dir.path().join("vehicles.json")).unwrap();
    let output: serde_json::Value = serde_json::from_str(&output_json).unwrap();

    // Should have only 1 vehicle (Tesla, not BYD)
    assert_eq!(output["vehicle_count"], 1);
}
