use ev_core::{Battery, Charging, Powertrain, Range, SlugName, Vehicle, VehicleType};
use ev_etl::output::postgresql::generate;
use std::io::Read;
use tempfile::NamedTempFile;

#[test]
fn test_postgresql_generation() {
    // Setup minimal vehicle
    use ev_core::Drivetrain;
    let v1 = Vehicle {
        schema_version: "1.0".into(),
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
    };

    let mut file = NamedTempFile::new().unwrap();
    let path = file.path();

    // Generate SQL file
    generate(&[v1], path).unwrap();

    // Verify content
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    assert!(content.contains("CREATE TABLE IF NOT EXISTS vehicles"));
    assert!(content.contains("INSERT INTO vehicles"));
    assert!(content.contains("tesla"));
    assert!(content.contains("model_3"));
}
