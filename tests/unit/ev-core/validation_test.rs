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
fn test_slug_name_accessors() {
    let slug_name = SlugName::new("tesla", "Tesla").unwrap();
    assert_eq!(slug_name.slug(), "tesla");
    assert_eq!(slug_name.name(), "Tesla");
}

#[test]
fn test_slug_name_display() {
    let slug_name = SlugName::new("model_3", "Model 3").unwrap();
    assert_eq!(format!("{}", slug_name), "Model 3");
}

#[test]
fn test_slug_name_validate_trait() {
    let slug_name = SlugName {
        slug: "valid_slug".to_string(),
        name: "Valid Name".to_string(),
    };
    assert!(slug_name.validate().is_ok());

    let invalid = SlugName {
        slug: "Invalid-Slug".to_string(),
        name: "Name".to_string(),
    };
    assert!(invalid.validate().is_err());

    let empty_name = SlugName {
        slug: "slug".to_string(),
        name: "".to_string(),
    };
    assert!(empty_name.validate().is_err());
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
fn test_year_value() {
    let year = Year::new(2024).unwrap();
    assert_eq!(year.value(), 2024);
}

#[test]
fn test_year_display() {
    let year = Year::new(2024).unwrap();
    assert_eq!(format!("{}", year), "2024");
}

#[test]
fn test_year_from_into() {
    let year = Year::new(2024).unwrap();
    let value: u16 = year.into();
    assert_eq!(value, 2024);
}

#[test]
fn test_year_try_from() {
    let result: Result<Year, _> = 2024u16.try_into();
    assert!(result.is_ok());
    assert_eq!(result.unwrap().value(), 2024);

    let invalid: Result<Year, _> = 1899u16.try_into();
    assert!(invalid.is_err());
}

#[test]
fn test_year_validate_trait() {
    let year = Year::new(2024).unwrap();
    assert!(year.validate().is_ok());
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
    let reparsed: ev_core::Vehicle =
        serde_json::from_str(&reserialized).expect("Failed to reparse");

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
    assert!(matches!(
        result,
        Err(ValidationError::MissingBatteryCapacity)
    ));
}

#[test]
fn test_vehicle_missing_charge_ports() {
    let json = r#"{
        "schema_version": "1.0.0",
        "make": {"slug": "test", "name": "Test"},
        "model": {"slug": "test", "name": "Test"},
        "year": 2024,
        "trim": {"slug": "base", "name": "Base"},
        "vehicle_type": "passenger_car",
        "powertrain": {"drivetrain": "fwd"},
        "battery": {"pack_capacity_kwh_net": 60.0},
        "charge_ports": [],
        "charging": {},
        "range": {"rated": [{"cycle": "wltp", "range_km": 300}]},
        "sources": [{"type": "oem", "title": "Test", "url": "https://test.com", "accessed_at": "2024-12-26T00:00:00Z"}]
    }"#;

    let vehicle: ev_core::Vehicle = serde_json::from_str(json).expect("Failed to parse");
    let result = vehicle.validate();

    assert!(result.is_err());
    assert!(matches!(result, Err(ValidationError::MissingChargePort)));
}

#[test]
fn test_vehicle_missing_range() {
    let json = r#"{
        "schema_version": "1.0.0",
        "make": {"slug": "test", "name": "Test"},
        "model": {"slug": "test", "name": "Test"},
        "year": 2024,
        "trim": {"slug": "base", "name": "Base"},
        "vehicle_type": "passenger_car",
        "powertrain": {"drivetrain": "fwd"},
        "battery": {"pack_capacity_kwh_net": 60.0},
        "charge_ports": [{"kind": "combo", "connector": "ccs2"}],
        "charging": {},
        "range": {"rated": []},
        "sources": [{"type": "oem", "title": "Test", "url": "https://test.com", "accessed_at": "2024-12-26T00:00:00Z"}]
    }"#;

    let vehicle: ev_core::Vehicle = serde_json::from_str(json).expect("Failed to parse");
    let result = vehicle.validate();

    assert!(result.is_err());
    assert!(matches!(result, Err(ValidationError::MissingRatedRange)));
}

#[test]
fn test_vehicle_missing_sources() {
    let json = r#"{
        "schema_version": "1.0.0",
        "make": {"slug": "test", "name": "Test"},
        "model": {"slug": "test", "name": "Test"},
        "year": 2024,
        "trim": {"slug": "base", "name": "Base"},
        "vehicle_type": "passenger_car",
        "powertrain": {"drivetrain": "fwd"},
        "battery": {"pack_capacity_kwh_net": 60.0},
        "charge_ports": [{"kind": "combo", "connector": "ccs2"}],
        "charging": {},
        "range": {"rated": [{"cycle": "wltp", "range_km": 300}]},
        "sources": []
    }"#;

    let vehicle: ev_core::Vehicle = serde_json::from_str(json).expect("Failed to parse");
    let result = vehicle.validate();

    assert!(result.is_err());
    assert!(matches!(result, Err(ValidationError::MissingSource)));
}

#[test]
fn test_validation_error_display() {
    let error = ValidationError::MissingBatteryCapacity;
    let display = format!("{}", error);
    assert!(!display.is_empty());

    let error = ValidationError::InvalidYear { value: 9999 };
    let display = format!("{}", error);
    assert!(display.contains("9999"));

    let error = ValidationError::InvalidSlug {
        value: "Invalid".to_string(),
    };
    let display = format!("{}", error);
    assert!(display.contains("Invalid"));
}

#[test]
fn test_validation_error_empty_value() {
    let error = ValidationError::empty_value("test_field");
    let display = format!("{}", error);
    assert!(display.contains("test_field") || !display.is_empty());
}

#[test]
fn test_validation_error_invalid_slug() {
    let error = ValidationError::invalid_slug("Bad-Slug");
    let display = format!("{}", error);
    assert!(display.contains("Bad-Slug"));
}

#[test]
fn test_vehicle_id_display() {
    use ev_core::VehicleId;

    let id = VehicleId::new("tesla", "model_3", 2024, "base", None).unwrap();
    assert_eq!(id.to_string(), "tesla:model_3:2024:base");

    let id_with_variant = VehicleId::new(
        "tesla",
        "model_3",
        2024,
        "base",
        Some("long_range".to_string()),
    )
    .unwrap();
    assert_eq!(
        id_with_variant.to_string(),
        "tesla:model_3:2024:base:long_range"
    );
}
