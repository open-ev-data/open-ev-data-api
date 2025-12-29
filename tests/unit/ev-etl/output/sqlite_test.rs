use ev_core::Drivetrain;
use ev_core::{Battery, Charging, Powertrain, Range, SlugName, Vehicle, VehicleType};
use ev_etl::output::sqlite::generate;
use rusqlite::Connection;
use tempfile::NamedTempFile;

fn create_test_vehicle() -> Vehicle {
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
        unique_code: Some("tesla:model_3:2024:model_3".to_string()),
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
fn test_sqlite_generation() {
    let v1 = create_test_vehicle();
    let file = NamedTempFile::new().unwrap();
    let path = file.path();

    // Generate SQLite DB
    generate(&[v1], path).unwrap();

    // Verify content
    let conn = Connection::open(path).unwrap();
    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM vehicles", [], |row| row.get(0))
        .unwrap();
    assert_eq!(count, 1);

    let make: String = conn
        .query_row("SELECT make_slug FROM vehicles LIMIT 1", [], |row| {
            row.get(0)
        })
        .unwrap();
    assert_eq!(make, "tesla");
}
