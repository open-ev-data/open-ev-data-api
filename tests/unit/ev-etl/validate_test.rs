use ev_core::{Battery, Charging, Powertrain, Range, SlugName, Vehicle, VehicleType};
use ev_etl::validate::{validate_all, validate_vehicle};

fn create_valid_vehicle() -> Vehicle {
    use ev_core::Drivetrain;

    Vehicle {
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
        charge_ports: vec![], // Invalid: Needs charge port
        charging: Charging::default(),
        range: Range {
            rated: vec![],
            real_world: None,
        }, // Invalid: Needs rated range
        sources: vec![], // Invalid: Needs source
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
fn test_validate_single_vehicle_fail() {
    let v = create_valid_vehicle();
    // It is incomplete, so it should fail
    assert!(validate_vehicle(&v).is_err());
}

#[test]
fn test_validate_all_mixed() {
    let v1 = create_valid_vehicle();
    let v2 = create_valid_vehicle();

    let results = validate_all(&[v1, v2]);
    assert_eq!(results.len(), 2);
    assert!(results[0].is_err());
}
