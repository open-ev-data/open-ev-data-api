use ev_core::Vehicle;
use ev_etl::output::json;
use tempfile::NamedTempFile;

fn create_test_vehicle() -> Vehicle {
    use ev_core::{
        Battery, Body, ChargePort, Charging, ConnectorType, Drivetrain, PortKind, Powertrain,
        Range, RangeCycle, RangeRated, SlugName, Source, SourceType, VehicleType,
    };

    Vehicle {
        schema_url: None,
        schema_version: "1.0.0".to_string(),
        make: SlugName {
            slug: "tesla".to_string(),
            name: "Tesla".to_string(),
        },
        model: SlugName {
            slug: "model_3".to_string(),
            name: "Model 3".to_string(),
        },
        year: 2024,
        trim: SlugName {
            slug: "long_range".to_string(),
            name: "Long Range".to_string(),
        },
        vehicle_type: VehicleType::PassengerCar,
        body: Some(Body {
            seats: Some(5),
            doors: Some(4),
            ..Default::default()
        }),
        powertrain: Powertrain {
            drivetrain: Drivetrain::Awd,
            system_power_kw: Some(300.0),
            system_torque_nm: None,
            motors: None,
            transmission: None,
        },
        battery: Battery {
            pack_capacity_kwh_net: Some(75.0),
            pack_capacity_kwh_gross: Some(82.0),
            ..Default::default()
        },
        charging: Charging::default(),
        charge_ports: vec![ChargePort {
            kind: PortKind::Combo,
            connector: ConnectorType::Ccs2,
            location: None,
            covers: None,
            light: None,
            motorized: None,
            notes: None,
        }],
        range: Range {
            rated: vec![RangeRated {
                cycle: RangeCycle::Wltp,
                range_km: 500.0,
                notes: None,
            }],
            real_world: None,
        },
        sources: vec![Source {
            source_type: SourceType::Oem,
            title: "Test".to_string(),
            url: "https://example.com".to_string(),
            accessed_at: "2024-01-01".to_string(),
            publisher: None,
            license: None,
            notes: None,
        }],
        unique_code: None,
        variant: None,
        markets: None,
        availability: None,
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
fn test_json_generate() {
    let vehicles = vec![create_test_vehicle(), create_test_vehicle()];
    let file = NamedTempFile::new().expect("Failed to create temp file");
    let path = file.path();

    // Call the actual module function
    json::generate(&vehicles, path).expect("Failed to generate JSON");

    // Verify content
    let content = std::fs::read_to_string(path).expect("Failed to read generated file");
    assert!(content.contains("tesla"));
    assert!(content.contains("model_3"));
    assert!(content.contains("vehicle_count\": 2"));

    // Verify checksum file
    let checksum_path = path.with_extension("json.sha256");
    assert!(checksum_path.exists());
    let checksum_content = std::fs::read_to_string(checksum_path).expect("Failed to read checksum");
    assert!(checksum_content.len() > 64);
}
