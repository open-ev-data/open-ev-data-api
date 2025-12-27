//! Shared test fixtures for unit tests.
//!
//! This module provides centralized test data following the DRY principle.
//! All test data is defined here and used across the test suite.

use ev_core::{
    Battery, ChargePort, Charging, ChargingAc, ChargingDc, ConnectorType, Drivetrain, PortKind,
    Powertrain, Range, RangeCycle, RangeRated, SlugName, Source, SourceType, Variant, Vehicle,
    VehicleType,
};
use std::path::PathBuf;

/// Returns the path to the fixtures directory.
pub fn fixtures_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("others")
        .join("fixtures")
}

/// Returns the path to the sample vehicle JSON file.
pub fn sample_vehicle_json_path() -> PathBuf {
    fixtures_dir().join("sample_vehicle.json")
}

/// Returns the path to the minimal sample vehicle JSON file.
pub fn sample_vehicle_minimal_json_path() -> PathBuf {
    fixtures_dir().join("sample_vehicle_minimal.json")
}

/// Creates a complete test vehicle with all fields populated.
pub fn create_complete_vehicle() -> Vehicle {
    Vehicle {
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
        variant: Some(Variant {
            slug: "awd".to_string(),
            name: "AWD".to_string(),
            kind: Some("drivetrain".to_string()),
            notes: None,
        }),
        powertrain: Powertrain {
            drivetrain: Drivetrain::Awd,
            system_power_kw: Some(324.0),
            system_torque_nm: Some(493.0),
            motors: None,
            transmission: None,
        },
        battery: Battery {
            pack_capacity_kwh_net: Some(75.0),
            pack_capacity_kwh_gross: Some(82.0),
            chemistry: Some("NMC".to_string()),
            manufacturer: Some("Panasonic".to_string()),
            heat_pump: Some(true),
            ..Default::default()
        },
        charge_ports: vec![ChargePort {
            kind: PortKind::Combo,
            connector: ConnectorType::Ccs2,
            location: None,
            covers: None,
            light: None,
            motorized: None,
            notes: None,
        }],
        charging: Charging {
            dc: Some(ChargingDc {
                max_power_kw: 250.0,
                voltage_range_v: None,
                max_current_a: Some(625.0),
                architecture_voltage_class: None,
                power_limits_by_voltage: None,
                notes: None,
            }),
            ac: Some(ChargingAc {
                max_power_kw: 11.0,
                supported_power_steps_kw: None,
                phases: Some(3),
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
                    range_km: 629.0,
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
        sources: vec![
            Source {
                source_type: SourceType::Oem,
                title: "Tesla Model 3 Specifications".to_string(),
                url: "https://www.tesla.com/model3".to_string(),
                accessed_at: "2024-12-01".to_string(),
                publisher: None,
                license: None,
                notes: None,
            },
            Source {
                source_type: SourceType::Regulatory,
                title: "EPA Fuel Economy".to_string(),
                url: "https://www.fueleconomy.gov".to_string(),
                accessed_at: "2024-12-01".to_string(),
                publisher: Some("EPA".to_string()),
                license: None,
                notes: None,
            },
        ],
        unique_code: Some("tesla_model_3_2024_long_range_awd".to_string()),
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

/// Creates a minimal test vehicle with only required fields.
pub fn create_minimal_vehicle() -> Vehicle {
    Vehicle {
        schema_version: "1.0.0".to_string(),
        make: SlugName {
            slug: "byd".to_string(),
            name: "BYD".to_string(),
        },
        model: SlugName {
            slug: "dolphin".to_string(),
            name: "Dolphin".to_string(),
        },
        year: 2024,
        trim: SlugName {
            slug: "comfort".to_string(),
            name: "Comfort".to_string(),
        },
        vehicle_type: VehicleType::PassengerCar,
        powertrain: Powertrain {
            drivetrain: Drivetrain::Fwd,
            system_power_kw: Some(70.0),
            system_torque_nm: None,
            motors: None,
            transmission: None,
        },
        battery: Battery {
            pack_capacity_kwh_net: Some(44.9),
            ..Default::default()
        },
        charge_ports: vec![ChargePort {
            kind: PortKind::Combo,
            connector: ConnectorType::Ccs2,
            location: None,
            covers: None,
            light: None,
            motorized: None,
            notes: None,
        }],
        charging: Charging::default(),
        range: Range {
            rated: vec![RangeRated {
                cycle: RangeCycle::Wltp,
                range_km: 340.0,
                notes: None,
            }],
            real_world: None,
        },
        sources: vec![Source {
            source_type: SourceType::Oem,
            title: "BYD Dolphin Specifications".to_string(),
            url: "https://www.byd.com/dolphin".to_string(),
            accessed_at: "2024-12-01".to_string(),
            publisher: None,
            license: None,
            notes: None,
        }],
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

/// Creates a list of test vehicles for batch testing.
pub fn create_vehicle_list() -> Vec<Vehicle> {
    vec![create_complete_vehicle(), create_minimal_vehicle()]
}

/// Sample vehicle JSON as a static string.
pub const SAMPLE_VEHICLE_JSON: &str = r#"{
    "schema_version": "1.0.0",
    "make": {"slug": "tesla", "name": "Tesla"},
    "model": {"slug": "model_3", "name": "Model 3"},
    "year": 2024,
    "trim": {"slug": "base", "name": "Base"},
    "vehicle_type": "passenger_car",
    "powertrain": {"drivetrain": "rwd", "system_power_kw": 208.0},
    "battery": {"pack_capacity_kwh_net": 60.0},
    "charge_ports": [{"kind": "combo", "connector": "ccs2"}],
    "charging": {},
    "range": {"rated": [{"cycle": "wltp", "range_km": 513.0}]},
    "sources": [{"type": "oem", "title": "Tesla", "url": "https://tesla.com", "accessed_at": "2024-01-01"}]
}"#;

/// Sample invalid vehicle JSON for testing error paths.
pub const INVALID_VEHICLE_JSON: &str = r#"{
    "schema_version": "1.0.0",
    "make": {"slug": "test", "name": "Test"},
    "model": {"slug": "test", "name": "Test"},
    "year": 2024,
    "trim": {"slug": "base", "name": "Base"},
    "vehicle_type": "passenger_car",
    "powertrain": {"drivetrain": "fwd"},
    "battery": {},
    "charge_ports": [],
    "charging": {},
    "range": {"rated": []},
    "sources": []
}"#;
