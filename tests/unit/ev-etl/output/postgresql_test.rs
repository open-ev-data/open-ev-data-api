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

#[test]
fn test_postgresql_with_variant() {
    use ev_core::{Drivetrain, Variant};
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
            system_power_kw: Some(208.0),
            system_torque_nm: Some(420.0),
            motors: None,
            transmission: None,
        },
        battery: Battery {
            pack_capacity_kwh_net: Some(60.0),
            pack_capacity_kwh_gross: Some(65.0),
            chemistry: Some("LFP".to_string()),
            ..Default::default()
        },
        charging: Charging::default(),
        range: Range {
            rated: vec![],
            real_world: None,
        },
        sources: vec![],
        charge_ports: vec![],
        unique_code: Some("tesla-model_3-2024-lr".to_string()),
        variant: Some(Variant {
            slug: "long_range".to_string(),
            name: "Long Range".to_string(),
            kind: None,
            notes: None,
        }),
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

    generate(&[v1], path).unwrap();

    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    assert!(content.contains("long_range"));
    assert!(content.contains("Long Range"));
    assert!(content.contains("208"));
    assert!(content.contains("420"));
    assert!(content.contains("LFP"));
}

#[test]
fn test_postgresql_with_performance() {
    use ev_core::{ChargingAc, ChargingDc, Drivetrain, Performance, RangeCycle, RangeRated};
    let v1 = Vehicle {
        schema_version: "1.0".into(),
        make: SlugName {
            slug: "tesla".into(),
            name: "Tesla".into(),
        },
        model: SlugName {
            slug: "model_s".into(),
            name: "Model S".into(),
        },
        year: 2024,
        trim: SlugName {
            slug: "plaid".into(),
            name: "Plaid".into(),
        },
        vehicle_type: VehicleType::PassengerCar,
        powertrain: Powertrain {
            drivetrain: Drivetrain::Awd,
            system_power_kw: Some(760.0),
            system_torque_nm: Some(1050.0),
            motors: None,
            transmission: None,
        },
        battery: Battery {
            pack_capacity_kwh_net: Some(95.0),
            ..Default::default()
        },
        charging: Charging {
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
        },
        range: Range {
            rated: vec![
                RangeRated {
                    cycle: RangeCycle::Wltp,
                    range_km: 600.0,
                    notes: None,
                },
                RangeRated {
                    cycle: RangeCycle::Epa,
                    range_km: 500.0,
                    notes: None,
                },
            ],
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
        performance: Some(Performance {
            acceleration_0_100_kmh_s: Some(2.1),
            acceleration_0_60_mph_s: Some(1.99),
            top_speed_kmh: Some(322.0),
            quarter_mile_s: Some(9.3),
        }),
        wheels_tires: None,
        pricing: None,
        software: None,
        links: None,
        images: None,
        metadata: None,
    };

    let mut file = NamedTempFile::new().unwrap();
    let path = file.path();

    generate(&[v1], path).unwrap();

    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    assert!(content.contains("2.1"));
    assert!(content.contains("322"));
    assert!(content.contains("250"));
    assert!(content.contains("11"));
    assert!(content.contains("600"));
    assert!(content.contains("500"));
}
