use ev_core::{Battery, Charging, Powertrain, Range, SlugName, Vehicle, VehicleType};

fn create_base_vehicle() -> Vehicle {
    use ev_core::Drivetrain;
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
fn test_vehicle_display_name() {
    let v = create_base_vehicle();
    assert_eq!(v.display_name(), "2024 Tesla Model Y Base");
}

#[test]
fn test_vehicle_usable_battery() {
    let v = create_base_vehicle();
    assert_eq!(v.usable_battery_kwh(), Some(60.0));
}

#[test]
fn test_vehicle_is_variant() {
    let mut v = create_base_vehicle();
    assert!(!v.is_variant());

    use ev_core::Variant;
    v.variant = Some(Variant {
        slug: "sport".into(),
        name: "Sport".into(),
        kind: None,
        notes: None,
    });
    assert!(v.is_variant());
}
