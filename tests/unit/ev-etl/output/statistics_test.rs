use ev_core::{Battery, Charging, Powertrain, Range, SlugName, Vehicle, VehicleType};
use ev_etl::output::statistics::generate;
use std::time::Duration;

// Helper function to create a minimal test vehicle
fn create_test_vehicle() -> Vehicle {
    use ev_core::Drivetrain;
    Vehicle {
        schema_url: None,
        schema_version: "1.0.0".to_string(),
        make: SlugName {
            slug: "tesla".into(),
            name: "Tesla".into(),
        },
        model: SlugName {
            slug: "model_3".into(),
            name: "Model 3".into(),
        },
        year: 2024,
        trim: SlugName {
            slug: "base".into(),
            name: "Base".into(),
        },
        vehicle_type: VehicleType::PassengerCar,
        // ... (minimal fields)
        powertrain: Powertrain {
            drivetrain: Drivetrain::Rwd,
            system_power_kw: None,
            system_torque_nm: None,
            motors: None,
            transmission: None,
        },
        battery: Battery {
            pack_capacity_kwh_net: Some(50.0),
            ..Default::default()
        },
        charging: Charging::default(),
        range: Range {
            rated: vec![],
            real_world: None,
        },
        sources: vec![],
        charge_ports: vec![],
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
fn test_statistics_generation() {
    let v1 = create_test_vehicle();
    let stats = generate(&[v1], Duration::from_secs(1));

    assert_eq!(stats.total_vehicles, 1);
    assert_eq!(stats.makes, 1);
    assert_eq!(stats.models, 1);
    assert_eq!(stats.year_range.min, 2024);
    assert_eq!(stats.year_range.max, 2024);
}
