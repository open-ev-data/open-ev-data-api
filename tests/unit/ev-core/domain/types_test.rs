use ev_core::{SlugName, VehicleId, Year};

#[test]
fn test_vehicle_id_canonical() {
    let id = VehicleId {
        make_slug: "tesla".to_string(),
        model_slug: "model_3".to_string(),
        year: 2024,
        trim_slug: "long_range".to_string(),
        variant_slug: Some("perf".to_string()),
    };
    assert_eq!(id.canonical_id(), "oed:tesla:model_3:2024:long_range:perf");

    let id_no_variant = VehicleId {
        make_slug: "tesla".to_string(),
        model_slug: "model_3".to_string(),
        year: 2024,
        trim_slug: "long_range".to_string(),
        variant_slug: None,
    };
    assert_eq!(
        id_no_variant.canonical_id(),
        "oed:tesla:model_3:2024:long_range"
    );
}

#[test]
fn test_slug_name() {
    let sn = SlugName {
        slug: "model_S".to_string(), // Usually slug should be lowercase, but struct holds string
        name: "Model S".to_string(),
    };
    assert_eq!(sn.slug, "model_S");
}

#[test]
fn test_year_validation() {
    assert!(Year::new(1899).is_err());
    assert!(Year::new(1900).is_ok());
    assert!(Year::new(2024).is_ok());
    assert!(Year::new(2100).is_ok()); // Assuming future years allowed
                                      // Check upper bound if exist
}
