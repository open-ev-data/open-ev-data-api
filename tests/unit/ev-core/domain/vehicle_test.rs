use ev_core::{
    Battery, ChargePort, Charging, ChargingAc, ChargingDc, ConnectorType, Drivetrain, PortKind,
    Powertrain, Range, RangeCycle, RangeRated, SlugName, Source, SourceType, Validate,
    ValidationError, Variant, Vehicle, VehicleType,
};

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
            slug: "base".to_string(),
            name: "Base".to_string(),
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
                range_km: 513.0,
                notes: None,
            }],
            real_world: None,
        },
        sources: vec![Source {
            source_type: SourceType::Oem,
            title: "Tesla Model 3 Specs".to_string(),
            url: "https://tesla.com/model3".to_string(),
            accessed_at: "2024-12-26T00:00:00Z".to_string(),
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

#[test]
fn test_vehicle_validation_success() {
    let vehicle = create_test_vehicle();
    assert!(vehicle.validate().is_ok());
}

#[test]
fn test_vehicle_display_name() {
    let vehicle = create_test_vehicle();
    assert_eq!(vehicle.display_name(), "2024 Tesla Model 3 Base");
}

#[test]
fn test_vehicle_id() {
    let vehicle = create_test_vehicle();
    let id = vehicle.id();
    assert_eq!(id.to_string(), "tesla:model_3:2024:base");
}

#[test]
fn test_vehicle_missing_battery() {
    let mut vehicle = create_test_vehicle();
    vehicle.battery = Battery::default();
    assert!(matches!(
        vehicle.validate(),
        Err(ValidationError::MissingBatteryCapacity)
    ));
}

#[test]
fn test_vehicle_missing_charge_ports() {
    let mut vehicle = create_test_vehicle();
    vehicle.charge_ports = vec![];
    assert!(matches!(
        vehicle.validate(),
        Err(ValidationError::MissingChargePort)
    ));
}

#[test]
fn test_vehicle_missing_range() {
    let mut vehicle = create_test_vehicle();
    vehicle.range.rated = vec![];
    assert!(matches!(
        vehicle.validate(),
        Err(ValidationError::MissingRatedRange)
    ));
}

#[test]
fn test_vehicle_missing_sources() {
    let mut vehicle = create_test_vehicle();
    vehicle.sources = vec![];
    assert!(matches!(
        vehicle.validate(),
        Err(ValidationError::MissingSource)
    ));
}

#[test]
fn test_vehicle_multiple_validation_errors() {
    let mut vehicle = create_test_vehicle();
    vehicle.battery = Battery::default();
    vehicle.charge_ports = vec![];
    vehicle.range.rated = vec![];
    vehicle.sources = vec![];
    assert!(matches!(
        vehicle.validate(),
        Err(ValidationError::Multiple(_))
    ));
}

#[test]
fn test_vehicle_is_variant_false() {
    let vehicle = create_test_vehicle();
    assert!(!vehicle.is_variant());
}

#[test]
fn test_vehicle_is_variant_true() {
    let mut vehicle = create_test_vehicle();
    vehicle.variant = Some(Variant {
        slug: "long_range".to_string(),
        name: "Long Range".to_string(),
        kind: None,
        notes: None,
    });
    assert!(vehicle.is_variant());
}

#[test]
fn test_vehicle_usable_battery_kwh() {
    let vehicle = create_test_vehicle();
    assert_eq!(vehicle.usable_battery_kwh(), Some(60.0));
}

#[test]
fn test_vehicle_usable_battery_kwh_none() {
    let mut vehicle = create_test_vehicle();
    vehicle.battery = Battery::default();
    assert_eq!(vehicle.usable_battery_kwh(), None);
}

#[test]
fn test_vehicle_wltp_range_km() {
    let vehicle = create_test_vehicle();
    assert_eq!(vehicle.wltp_range_km(), Some(513.0));
}

#[test]
fn test_vehicle_epa_range_km_none() {
    let vehicle = create_test_vehicle();
    assert_eq!(vehicle.epa_range_km(), None);
}

#[test]
fn test_vehicle_epa_range_km_some() {
    let mut vehicle = create_test_vehicle();
    vehicle.range.rated.push(RangeRated {
        cycle: RangeCycle::Epa,
        range_km: 400.0,
        notes: None,
    });
    assert_eq!(vehicle.epa_range_km(), Some(400.0));
}

#[test]
fn test_vehicle_max_dc_power_kw_none() {
    let vehicle = create_test_vehicle();
    assert_eq!(vehicle.max_dc_power_kw(), None);
}

#[test]
fn test_vehicle_max_dc_power_kw_some() {
    let mut vehicle = create_test_vehicle();
    vehicle.charging.dc = Some(ChargingDc {
        max_power_kw: 250.0,
        voltage_range_v: None,
        max_current_a: None,
        architecture_voltage_class: None,
        power_limits_by_voltage: None,
        notes: None,
    });
    assert_eq!(vehicle.max_dc_power_kw(), Some(250.0));
}

#[test]
fn test_vehicle_max_ac_power_kw_none() {
    let vehicle = create_test_vehicle();
    assert_eq!(vehicle.max_ac_power_kw(), None);
}

#[test]
fn test_vehicle_max_ac_power_kw_some() {
    let mut vehicle = create_test_vehicle();
    vehicle.charging.ac = Some(ChargingAc {
        max_power_kw: 11.0,
        supported_power_steps_kw: None,
        phases: None,
        voltage_range_v: None,
        frequency_hz: None,
        max_current_a: None,
        onboard_charger_count: None,
        notes: None,
    });
    assert_eq!(vehicle.max_ac_power_kw(), Some(11.0));
}

#[test]
fn test_vehicle_invalid_make_slug() {
    let mut vehicle = create_test_vehicle();
    vehicle.make.slug = "INVALID".to_string();
    assert!(vehicle.validate().is_err());
}

#[test]
fn test_vehicle_invalid_year() {
    let mut vehicle = create_test_vehicle();
    vehicle.year = 1800;
    assert!(vehicle.validate().is_err());
}
