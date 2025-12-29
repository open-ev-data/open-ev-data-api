use ev_core::{SlugName, VehicleId, Year};

#[test]
fn test_slug_name_valid() {
    let slug_name = SlugName::new("tesla", "Tesla").unwrap();
    assert_eq!(slug_name.slug(), "tesla");
    assert_eq!(slug_name.name(), "Tesla");
}

#[test]
fn test_slug_name_invalid_slug() {
    assert!(SlugName::new("Tesla", "Tesla").is_err());
    assert!(SlugName::new("", "Tesla").is_err());
}

#[test]
fn test_slug_name_empty_name() {
    assert!(SlugName::new("tesla", "").is_err());
}

#[test]
fn test_year_valid() {
    assert!(Year::new(2024).is_ok());
    assert!(Year::new(1900).is_ok());
    assert!(Year::new(2100).is_ok());
}

#[test]
fn test_year_invalid() {
    assert!(Year::new(1899).is_err());
    assert!(Year::new(2101).is_err());
}

#[test]
fn test_vehicle_id_display() {
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

#[test]
fn test_vehicle_id_struct_fields() {
    let id = VehicleId {
        make_slug: "tesla".to_string(),
        model_slug: "model_3".to_string(),
        year: 2024,
        trim_slug: "long_range".to_string(),
        variant_slug: Some("perf".to_string()),
    };
    assert_eq!(id.to_string(), "tesla:model_3:2024:long_range:perf");
}
