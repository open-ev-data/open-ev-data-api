use ev_core::{
    Battery, Body, ChargePort, Charging, ConnectorType, Drivetrain, PortKind, Powertrain, Range,
    RangeCycle, RangeRated, SlugName, Source, SourceType, Variant, Vehicle, VehicleType,
};
use ev_etl::output::csv;
use tempfile::NamedTempFile;

fn create_test_vehicle() -> Vehicle {
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
            system_torque_nm: Some(500.0),
            motors: None,
            transmission: None,
        },
        battery: Battery {
            pack_capacity_kwh_net: Some(75.0),
            pack_capacity_kwh_gross: Some(82.0),
            chemistry: Some("NMC".to_string()),
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
            rated: vec![
                RangeRated {
                    cycle: RangeCycle::Wltp,
                    range_km: 500.0,
                    notes: None,
                },
                RangeRated {
                    cycle: RangeCycle::Epa,
                    range_km: 450.0,
                    notes: None,
                },
            ],
            real_world: None,
        },
        sources: vec![Source {
            source_type: SourceType::Oem,
            title: "Tesla Official".to_string(),
            url: "https://tesla.com".to_string(),
            accessed_at: "2024-01-01".to_string(),
            publisher: None,
            license: None,
            notes: None,
        }],
        unique_code: Some("tesla_model_3_2024_long_range".to_string()),
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

fn create_minimal_vehicle() -> Vehicle {
    Vehicle {
        schema_url: None,
        schema_version: "1.0.0".to_string(),
        make: SlugName {
            slug: "bmw".to_string(),
            name: "BMW".to_string(),
        },
        model: SlugName {
            slug: "i4".to_string(),
            name: "i4".to_string(),
        },
        year: 2024,
        trim: SlugName {
            slug: "base".to_string(),
            name: "Base".to_string(),
        },
        vehicle_type: VehicleType::PassengerCar,
        body: None,
        powertrain: Powertrain {
            drivetrain: Drivetrain::Rwd,
            system_power_kw: None,
            system_torque_nm: None,
            motors: None,
            transmission: None,
        },
        battery: Battery::default(),
        charging: Charging::default(),
        charge_ports: vec![],
        range: Range {
            rated: vec![],
            real_world: None,
        },
        sources: vec![],
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
fn test_csv_generate() {
    let vehicles = vec![create_test_vehicle()];
    let file = NamedTempFile::new().expect("Failed to create temp file");
    let path = file.path();

    csv::generate(&vehicles, path).expect("Failed to generate CSV");

    let content = std::fs::read_to_string(path).expect("Failed to read generated file");

    assert!(content.contains("tesla"));
    assert!(content.contains("model_3"));
    assert!(content.contains("Tesla"));
    assert!(content.contains("Model 3"));
    assert!(content.contains("2024"));
    assert!(content.contains("long_range"));
}

#[test]
fn test_csv_generate_multiple_vehicles() {
    let vehicles = vec![create_test_vehicle(), create_minimal_vehicle()];
    let file = NamedTempFile::new().expect("Failed to create temp file");
    let path = file.path();

    csv::generate(&vehicles, path).expect("Failed to generate CSV");

    let content = std::fs::read_to_string(path).expect("Failed to read generated file");

    assert!(content.contains("tesla"));
    assert!(content.contains("bmw"));
    assert!(content.contains("model_3"));
    assert!(content.contains("i4"));
}

#[test]
fn test_csv_generate_empty() {
    let vehicles: Vec<Vehicle> = vec![];
    let file = NamedTempFile::new().expect("Failed to create temp file");
    let path = file.path();

    csv::generate(&vehicles, path).expect("Failed to generate CSV");

    let content = std::fs::read_to_string(path).expect("Failed to read generated file");

    assert!(content.contains("unique_code"));
    assert!(content.contains("make_slug"));

    let lines: Vec<&str> = content.lines().collect();
    assert_eq!(lines.len(), 1);
}

#[test]
fn test_csv_header_columns() {
    let vehicles = vec![create_test_vehicle()];
    let file = NamedTempFile::new().expect("Failed to create temp file");
    let path = file.path();

    csv::generate(&vehicles, path).expect("Failed to generate CSV");

    let content = std::fs::read_to_string(path).expect("Failed to read generated file");
    let header = content.lines().next().expect("No header found");

    assert!(header.contains("unique_code"));
    assert!(header.contains("make_slug"));
    assert!(header.contains("make_name"));
    assert!(header.contains("model_slug"));
    assert!(header.contains("model_name"));
    assert!(header.contains("year"));
    assert!(header.contains("trim_slug"));
    assert!(header.contains("trim_name"));
    assert!(header.contains("variant_slug"));
    assert!(header.contains("variant_name"));
    assert!(header.contains("vehicle_type"));
    assert!(header.contains("drivetrain"));
    assert!(header.contains("system_power_kw"));
    assert!(header.contains("system_torque_nm"));
    assert!(header.contains("battery_capacity_gross_kwh"));
    assert!(header.contains("battery_capacity_net_kwh"));
    assert!(header.contains("battery_chemistry"));
    assert!(header.contains("dc_max_power_kw"));
    assert!(header.contains("ac_max_power_kw"));
    assert!(header.contains("range_wltp_km"));
    assert!(header.contains("range_epa_km"));
    assert!(header.contains("acceleration_0_100_s"));
    assert!(header.contains("top_speed_kmh"));
    assert!(header.contains("charge_connectors"));
    assert!(header.contains("sources"));
}

#[test]
fn test_csv_data_values() {
    let vehicles = vec![create_test_vehicle()];
    let file = NamedTempFile::new().expect("Failed to create temp file");
    let path = file.path();

    csv::generate(&vehicles, path).expect("Failed to generate CSV");

    let content = std::fs::read_to_string(path).expect("Failed to read generated file");
    let lines: Vec<&str> = content.lines().collect();

    assert!(lines.len() >= 2);

    let data_line = lines[1];
    assert!(data_line.contains("tesla_model_3_2024_long_range"));
    assert!(data_line.contains("300"));
    assert!(data_line.contains("500"));
    assert!(data_line.contains("82"));
    assert!(data_line.contains("75"));
    assert!(data_line.contains("NMC"));
}

#[test]
fn test_csv_unique_code_fallback() {
    let mut vehicle = create_minimal_vehicle();
    vehicle.unique_code = None;

    let vehicles = vec![vehicle];
    let file = NamedTempFile::new().expect("Failed to create temp file");
    let path = file.path();

    csv::generate(&vehicles, path).expect("Failed to generate CSV");

    let content = std::fs::read_to_string(path).expect("Failed to read generated file");
    let lines: Vec<&str> = content.lines().collect();

    assert!(lines.len() >= 2);

    let data_line = lines[1];
    assert!(data_line.contains("bmw"));
    assert!(data_line.contains("i4"));
}

#[test]
fn test_csv_with_variant() {
    let mut vehicle = create_test_vehicle();
    vehicle.variant = Some(Variant {
        slug: "performance".to_string(),
        name: "Performance".to_string(),
        kind: None,
        notes: None,
    });

    let vehicles = vec![vehicle];
    let file = NamedTempFile::new().expect("Failed to create temp file");
    let path = file.path();

    csv::generate(&vehicles, path).expect("Failed to generate CSV");

    let content = std::fs::read_to_string(path).expect("Failed to read generated file");

    assert!(content.contains("performance"));
    assert!(content.contains("Performance"));
}

#[test]
fn test_csv_connector_formatting() {
    let vehicles = vec![create_test_vehicle()];
    let file = NamedTempFile::new().expect("Failed to create temp file");
    let path = file.path();

    csv::generate(&vehicles, path).expect("Failed to generate CSV");

    let content = std::fs::read_to_string(path).expect("Failed to read generated file");

    assert!(content.contains("Ccs2"));
}

#[test]
fn test_csv_sources_formatting() {
    let vehicles = vec![create_test_vehicle()];
    let file = NamedTempFile::new().expect("Failed to create temp file");
    let path = file.path();

    csv::generate(&vehicles, path).expect("Failed to generate CSV");

    let content = std::fs::read_to_string(path).expect("Failed to read generated file");

    assert!(content.contains("https://tesla.com"));
}

#[test]
fn test_csv_empty_optional_fields() {
    let vehicles = vec![create_minimal_vehicle()];
    let file = NamedTempFile::new().expect("Failed to create temp file");
    let path = file.path();

    csv::generate(&vehicles, path).expect("Failed to generate CSV");

    let content = std::fs::read_to_string(path).expect("Failed to read generated file");
    let lines: Vec<&str> = content.lines().collect();

    assert!(lines.len() >= 2);
}

#[test]
fn test_csv_with_charging_data() {
    use ev_core::{ChargingAc, ChargingDc};

    let mut vehicle = create_test_vehicle();
    vehicle.charging = Charging {
        dc: Some(ChargingDc {
            max_power_kw: 250.0,
            voltage_range_v: None,
            max_current_a: None,
            architecture_voltage_class: None,
            power_limits_by_voltage: None,
            notes: None,
        }),
        ac: Some(ChargingAc {
            max_power_kw: 11.0,
            supported_power_steps_kw: None,
            phases: None,
            voltage_range_v: None,
            frequency_hz: None,
            max_current_a: None,
            onboard_charger_count: None,
            notes: None,
        }),
        protocols: None,
        dc_charge_curve: None,
        charging_time: None,
    };

    let vehicles = vec![vehicle];
    let file = NamedTempFile::new().expect("Failed to create temp file");
    let path = file.path();

    csv::generate(&vehicles, path).expect("Failed to generate CSV");

    let content = std::fs::read_to_string(path).expect("Failed to read generated file");

    assert!(content.contains("250"));
    assert!(content.contains("11"));
}

#[test]
fn test_csv_with_performance_data() {
    use ev_core::Performance;

    let mut vehicle = create_test_vehicle();
    vehicle.performance = Some(Performance {
        acceleration_0_100_kmh_s: Some(5.8),
        acceleration_0_60_mph_s: Some(5.2),
        top_speed_kmh: Some(225.0),
        quarter_mile_s: Some(14.5),
    });

    let vehicles = vec![vehicle];
    let file = NamedTempFile::new().expect("Failed to create temp file");
    let path = file.path();

    csv::generate(&vehicles, path).expect("Failed to generate CSV");

    let content = std::fs::read_to_string(path).expect("Failed to read generated file");

    assert!(content.contains("5.8"));
    assert!(content.contains("225"));
}
