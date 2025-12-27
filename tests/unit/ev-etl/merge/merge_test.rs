use ev_etl::ingest::{FileType, VehicleFile, parse_json_file};
use ev_etl::merge::deep_merge;
use serde_json::json;
use std::path::PathBuf;
use tempfile::TempDir;

fn create_test_dataset() -> TempDir {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let base = temp_dir.path();

    // Create make/model structure
    std::fs::create_dir_all(base.join("tesla/model_3/2024")).expect("Failed to create dirs");

    // Create year base file
    let year_base = json!({
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
        serde_json::to_string_pretty(&year_base).unwrap(),
    )
    .expect("Failed to write year base");

    // Create variant file
    let variant = json!({
        "schema_version": "1.0.0",
        "make": {"slug": "tesla", "name": "Tesla"},
        "model": {"slug": "model_3", "name": "Model 3"},
        "year": 2024,
        "trim": {"slug": "long_range", "name": "Long Range"},
        "variant": {"slug": "awd", "name": "AWD"},
        "vehicle_type": "passenger_car",
        "powertrain": {"drivetrain": "awd", "system_power_kw": 324.0},
        "battery": {"pack_capacity_kwh_net": 75.0},
        "charge_ports": [{"kind": "combo", "connector": "ccs2"}],
        "charging": {"dc": {"max_power_kw": 250.0}},
        "range": {"rated": [{"cycle": "wltp", "range_km": 629.0}]},
        "sources": [{"type": "oem", "title": "Tesla", "url": "https://tesla.com", "accessed_at": "2024-01-01"}]
    });

    std::fs::write(
        base.join("tesla/model_3/2024/tesla_model_3_long_range.json"),
        serde_json::to_string_pretty(&variant).unwrap(),
    )
    .expect("Failed to write variant");

    temp_dir
}

#[test]
fn test_merge_all_with_temp_dataset() {
    let temp_dir = create_test_dataset();
    let files = ev_etl::ingest::load_dataset(temp_dir.path()).expect("Failed to load dataset");

    assert!(!files.is_empty());

    let vehicles = ev_etl::merge::merge_all(&files).expect("Failed to merge");

    assert!(!vehicles.is_empty());
    assert!(vehicles.iter().any(|v| v.make.slug == "tesla"));
}

#[test]
fn test_load_dataset_empty_dir() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let files = ev_etl::ingest::load_dataset(temp_dir.path()).expect("Failed to load dataset");

    assert!(files.is_empty());
}

#[test]
fn test_load_dataset_with_valid_structure() {
    let temp_dir = create_test_dataset();
    let files = ev_etl::ingest::load_dataset(temp_dir.path()).expect("Failed to load dataset");

    assert_eq!(files.len(), 2);

    let year_base = files.iter().find(|f| f.file_type == FileType::YearBase);
    assert!(year_base.is_some());
    assert_eq!(year_base.unwrap().make_slug, "tesla");
    assert_eq!(year_base.unwrap().model_slug, "model_3");
    assert_eq!(year_base.unwrap().year, Some(2024));

    let variant = files.iter().find(|f| f.file_type == FileType::Variant);
    assert!(variant.is_some());
}

#[test]
fn test_merge_all_empty() {
    let files: Vec<VehicleFile> = vec![];
    let vehicles = ev_etl::merge::merge_all(&files).expect("Failed to merge");
    assert!(vehicles.is_empty());
}

#[test]
fn test_merge_all_preserves_vehicle_data() {
    let temp_dir = create_test_dataset();
    let files = ev_etl::ingest::load_dataset(temp_dir.path()).expect("Failed to load dataset");
    let vehicles = ev_etl::merge::merge_all(&files).expect("Failed to merge");

    for vehicle in &vehicles {
        assert_eq!(vehicle.make.slug, "tesla");
        assert_eq!(vehicle.model.slug, "model_3");
        assert_eq!(vehicle.year, 2024);
    }
}

#[test]
fn test_merge_all_variant_overrides() {
    let temp_dir = create_test_dataset();
    let files = ev_etl::ingest::load_dataset(temp_dir.path()).expect("Failed to load dataset");
    let vehicles = ev_etl::merge::merge_all(&files).expect("Failed to merge");

    // Find the long range variant
    let long_range = vehicles.iter().find(|v| v.trim.slug == "long_range");

    if let Some(lr) = long_range {
        assert_eq!(lr.powertrain.system_power_kw, Some(324.0));
    }
}

#[test]
fn test_vehicle_file_debug() {
    let file = VehicleFile {
        path: PathBuf::from("/test/path.json"),
        make_slug: "tesla".to_string(),
        model_slug: "model_3".to_string(),
        year: Some(2024),
        file_type: FileType::YearBase,
        content: json!({}),
    };

    let debug_str = format!("{:?}", file);
    assert!(debug_str.contains("tesla"));
    assert!(debug_str.contains("model_3"));
}

#[test]
fn test_file_type_ordering() {
    assert!(FileType::ModelBase < FileType::YearBase);
    assert!(FileType::YearBase < FileType::Variant);
    assert!(FileType::ModelBase < FileType::Variant);
}

#[test]
fn test_file_type_partial_cmp() {
    assert!(
        FileType::ModelBase
            .partial_cmp(&FileType::YearBase)
            .is_some()
    );
    assert_eq!(
        FileType::YearBase.partial_cmp(&FileType::YearBase),
        Some(std::cmp::Ordering::Equal)
    );
}

#[test]
fn test_file_type_debug() {
    assert_eq!(format!("{:?}", FileType::ModelBase), "ModelBase");
    assert_eq!(format!("{:?}", FileType::YearBase), "YearBase");
    assert_eq!(format!("{:?}", FileType::Variant), "Variant");
}

#[test]
fn test_parse_json_file_valid() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let file_path = temp_dir.path().join("test.json");

    std::fs::write(&file_path, r#"{"key": "value"}"#).expect("Failed to write");

    let result = parse_json_file(&file_path);
    assert!(result.is_ok());
}

#[test]
fn test_parse_json_file_invalid() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let file_path = temp_dir.path().join("invalid.json");

    std::fs::write(&file_path, "not valid json {").expect("Failed to write");

    let result = parse_json_file(&file_path);
    assert!(result.is_err());
}

#[test]
fn test_parse_json_file_not_found() {
    let result = parse_json_file(std::path::Path::new("/nonexistent/file.json"));
    assert!(result.is_err());
}

#[test]
fn test_deep_merge_preserves_base_fields() {
    let base = json!({
        "make": {"slug": "tesla", "name": "Tesla"},
        "year": 2024
    });
    let overlay = json!({
        "variant": {"slug": "lr", "name": "Long Range"}
    });

    let result = deep_merge(&base, &overlay);

    assert_eq!(result["make"]["slug"], "tesla");
    assert_eq!(result["year"], 2024);
    assert_eq!(result["variant"]["slug"], "lr");
}

#[test]
fn test_deep_merge_override_nested() {
    let base = json!({
        "battery": {"capacity": 60, "chemistry": "nmc"}
    });
    let overlay = json!({
        "battery": {"capacity": 75}
    });

    let result = deep_merge(&base, &overlay);

    assert_eq!(result["battery"]["capacity"], 75);
    assert_eq!(result["battery"]["chemistry"], "nmc");
}

#[test]
fn test_merge_all_invalid_base_vehicle() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let base = temp_dir.path();

    std::fs::create_dir_all(base.join("tesla/model_3/2024")).expect("Failed to create dirs");

    let invalid_base = json!({
        "schema_version": "1.0.0",
        "model": {"slug": "model_3", "name": "Model 3"},
        "year": 2024
    });

    std::fs::write(
        base.join("tesla/model_3/2024/tesla_model_3.json"),
        serde_json::to_string_pretty(&invalid_base).unwrap(),
    )
    .expect("Failed to write");

    let files = ev_etl::ingest::load_dataset(base).expect("Failed to load dataset");

    let vehicles = ev_etl::merge::merge_all(&files).expect("Failed to merge");

    assert!(vehicles.is_empty());
}

#[test]
fn test_merge_all_invalid_variant_vehicle() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let base = temp_dir.path();

    std::fs::create_dir_all(base.join("tesla/model_3/2024")).expect("Failed to create dirs");

    let valid_base = json!({
        "schema_version": "1.0.0",
        "make": {"slug": "tesla", "name": "Tesla"},
        "model": {"slug": "model_3", "name": "Model 3"},
        "year": 2024,
        "trim": {"slug": "base", "name": "Base"},
        "vehicle_type": "passenger_car",
        "powertrain": {"drivetrain": "rwd"},
        "battery": {},
        "charging": {},
        "range": {"rated":[]},
        "sources": [],
        "charge_ports": []
    });

    std::fs::write(
        base.join("tesla/model_3/2024/tesla_model_3.json"),
        serde_json::to_string_pretty(&valid_base).unwrap(),
    )
    .expect("Failed to write base");

    let invalid_variant = json!({
        "trim": {"slug": "long_range", "name": "Long Range"},
        "vehicle_type": null
    });

    std::fs::write(
        base.join("tesla/model_3/2024/tesla_model_3_invalid.json"),
        serde_json::to_string_pretty(&invalid_variant).unwrap(),
    )
    .expect("Failed to write variant");

    let files = ev_etl::ingest::load_dataset(base).expect("Failed to load dataset");

    let vehicles = ev_etl::merge::merge_all(&files).expect("Failed to merge");

    assert_eq!(vehicles.len(), 1);
    assert_eq!(vehicles[0].trim.slug, "base");
}
