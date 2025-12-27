use ev_core::{SlugName, Validate, ValidationError, Year};

#[test]
fn test_slug_name_validation() {
    let valid = SlugName::new("tesla", "Tesla");
    assert!(valid.is_ok());

    let invalid_slug = SlugName::new("Tesla", "Tesla");
    assert!(invalid_slug.is_err());

    let empty_slug = SlugName::new("", "Tesla");
    assert!(empty_slug.is_err());

    let empty_name = SlugName::new("tesla", "");
    assert!(empty_name.is_err());
}

#[test]
fn test_year_validation() {
    assert!(Year::new(2024).is_ok());
    assert!(Year::new(1900).is_ok());
    assert!(Year::new(2100).is_ok());

    assert!(Year::new(1899).is_err());
    assert!(Year::new(2101).is_err());
    assert!(Year::new(0).is_err());
}

#[test]
fn test_vehicle_serialization_roundtrip() {
    let json = r#"{
        "schema_version": "1.0.0",
        "make": {"slug": "byd", "name": "BYD"},
        "model": {"slug": "dolphin", "name": "Dolphin"},
        "year": 2024,
        "trim": {"slug": "base", "name": "Base"},
        "vehicle_type": "passenger_car",
        "powertrain": {"drivetrain": "fwd"},
        "battery": {"pack_capacity_kwh_net": 44.9},
        "charge_ports": [{"kind": "combo", "connector": "ccs2"}],
        "charging": {},
        "range": {"rated": [{"cycle": "wltp", "range_km": 340}]},
        "sources": [{"type": "oem", "title": "BYD", "url": "https://byd.com", "accessed_at": "2024-12-26T00:00:00Z"}]
    }"#;

    let vehicle: ev_core::Vehicle = serde_json::from_str(json).expect("Failed to deserialize");

    assert_eq!(vehicle.make.slug, "byd");
    assert_eq!(vehicle.model.name, "Dolphin");
    assert_eq!(vehicle.year, 2024);

    let reserialized = serde_json::to_string(&vehicle).expect("Failed to serialize");
    let reparsed: ev_core::Vehicle = serde_json::from_str(&reserialized).expect("Failed to reparse");

    assert_eq!(vehicle.make.slug, reparsed.make.slug);
    assert_eq!(vehicle.year, reparsed.year);
}

#[test]
fn test_vehicle_validation() {
    let json = r#"{
        "schema_version": "1.0.0",
        "make": {"slug": "tesla", "name": "Tesla"},
        "model": {"slug": "model_3", "name": "Model 3"},
        "year": 2024,
        "trim": {"slug": "base", "name": "Base"},
        "vehicle_type": "passenger_car",
        "powertrain": {"drivetrain": "rwd"},
        "battery": {"pack_capacity_kwh_net": 60.0},
        "charge_ports": [{"kind": "combo", "connector": "ccs2"}],
        "charging": {},
        "range": {"rated": [{"cycle": "wltp", "range_km": 513}]},
        "sources": [{"type": "oem", "title": "Tesla", "url": "https://tesla.com", "accessed_at": "2024-12-26T00:00:00Z"}]
    }"#;

    let vehicle: ev_core::Vehicle = serde_json::from_str(json).expect("Failed to parse");
    assert!(vehicle.validate().is_ok());
}

#[test]
fn test_vehicle_missing_battery_capacity() {
    let json = r#"{
        "schema_version": "1.0.0",
        "make": {"slug": "test", "name": "Test"},
        "model": {"slug": "test", "name": "Test"},
        "year": 2024,
        "trim": {"slug": "base", "name": "Base"},
        "vehicle_type": "passenger_car",
        "powertrain": {"drivetrain": "fwd"},
        "battery": {},
        "charge_ports": [{"kind": "combo", "connector": "ccs2"}],
        "charging": {},
        "range": {"rated": [{"cycle": "wltp", "range_km": 300}]},
        "sources": [{"type": "oem", "title": "Test", "url": "https://test.com", "accessed_at": "2024-12-26T00:00:00Z"}]
    }"#;

    let vehicle: ev_core::Vehicle = serde_json::from_str(json).expect("Failed to parse");
    let result = vehicle.validate();

    assert!(result.is_err());
    assert!(matches!(result, Err(ValidationError::MissingBatteryCapacity)));
}
