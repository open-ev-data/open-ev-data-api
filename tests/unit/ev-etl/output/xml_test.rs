use ev_core::{
    Battery, Body, ChargePort, Charging, ConnectorType, Drivetrain, PortKind, Powertrain, Range,
    RangeCycle, RangeRated, SlugName, Source, SourceType, Vehicle, VehicleType,
};
use ev_etl::output::xml;
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
            rated: vec![RangeRated {
                cycle: RangeCycle::Wltp,
                range_km: 500.0,
                notes: None,
            }],
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
        unique_code: Some("tesla:model_3:2024:long_range".to_string()),
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
        unique_code: Some("bmw:i4:2024:i4".to_string()),
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
fn test_xml_generate() {
    let vehicles = vec![create_test_vehicle()];
    let file = NamedTempFile::new().expect("Failed to create temp file");
    let path = file.path();

    xml::generate(&vehicles, path).expect("Failed to generate XML");

    let content = std::fs::read_to_string(path).expect("Failed to read generated file");

    assert!(content.contains(r#"<?xml version="1.0" encoding="UTF-8"?>"#));
    assert!(content.contains("<vehicles"));
    assert!(content.contains("</vehicles>"));
}

#[test]
fn test_xml_generate_vehicle_elements() {
    let vehicles = vec![create_test_vehicle()];
    let file = NamedTempFile::new().expect("Failed to create temp file");
    let path = file.path();

    xml::generate(&vehicles, path).expect("Failed to generate XML");

    let content = std::fs::read_to_string(path).expect("Failed to read generated file");

    assert!(content.contains("<vehicle"));
    assert!(content.contains("</vehicle>"));
    assert!(content.contains("<make"));
    assert!(content.contains("</make>"));
    assert!(content.contains("<model"));
    assert!(content.contains("</model>"));
    assert!(content.contains("<year>"));
    assert!(content.contains("</year>"));
    assert!(content.contains("<trim"));
    assert!(content.contains("</trim>"));
}

#[test]
fn test_xml_generate_multiple_vehicles() {
    let vehicles = vec![create_test_vehicle(), create_minimal_vehicle()];
    let file = NamedTempFile::new().expect("Failed to create temp file");
    let path = file.path();

    xml::generate(&vehicles, path).expect("Failed to generate XML");

    let content = std::fs::read_to_string(path).expect("Failed to read generated file");

    assert!(content.contains("Tesla"));
    assert!(content.contains("BMW"));
    assert!(content.contains("Model 3"));
    assert!(content.contains("i4"));
    assert!(content.contains(r#"count="2""#));
}

#[test]
fn test_xml_generate_empty() {
    let vehicles: Vec<Vehicle> = vec![];
    let file = NamedTempFile::new().expect("Failed to create temp file");
    let path = file.path();

    xml::generate(&vehicles, path).expect("Failed to generate XML");

    let content = std::fs::read_to_string(path).expect("Failed to read generated file");

    assert!(content.contains(r#"<?xml version="1.0" encoding="UTF-8"?>"#));
    assert!(content.contains(r#"count="0""#));
}

#[test]
fn test_xml_vehicle_id_attribute() {
    let vehicles = vec![create_test_vehicle()];
    let file = NamedTempFile::new().expect("Failed to create temp file");
    let path = file.path();

    xml::generate(&vehicles, path).expect("Failed to generate XML");

    let content = std::fs::read_to_string(path).expect("Failed to read generated file");

    assert!(content.contains(r#"<vehicle id="tesla:model_3:2024:long_range">"#));
}

#[test]
fn test_xml_make_model_attributes() {
    let vehicles = vec![create_test_vehicle()];
    let file = NamedTempFile::new().expect("Failed to create temp file");
    let path = file.path();

    xml::generate(&vehicles, path).expect("Failed to generate XML");

    let content = std::fs::read_to_string(path).expect("Failed to read generated file");

    assert!(content.contains(r#"slug="tesla""#));
    assert!(content.contains(r#"slug="model_3""#));
    assert!(content.contains(">Tesla</make>"));
    assert!(content.contains(">Model 3</model>"));
}

#[test]
fn test_xml_powertrain_section() {
    let vehicles = vec![create_test_vehicle()];
    let file = NamedTempFile::new().expect("Failed to create temp file");
    let path = file.path();

    xml::generate(&vehicles, path).expect("Failed to generate XML");

    let content = std::fs::read_to_string(path).expect("Failed to read generated file");

    assert!(content.contains("<powertrain>"));
    assert!(content.contains("</powertrain>"));
    assert!(content.contains("<drivetrain>"));
    assert!(content.contains("</drivetrain>"));
    assert!(content.contains("<systemPowerKw>300</systemPowerKw>"));
    assert!(content.contains("<systemTorqueNm>500</systemTorqueNm>"));
}

#[test]
fn test_xml_battery_section() {
    let vehicles = vec![create_test_vehicle()];
    let file = NamedTempFile::new().expect("Failed to create temp file");
    let path = file.path();

    xml::generate(&vehicles, path).expect("Failed to generate XML");

    let content = std::fs::read_to_string(path).expect("Failed to read generated file");

    assert!(content.contains("<battery>"));
    assert!(content.contains("</battery>"));
    assert!(content.contains("<packCapacityKwhGross>82</packCapacityKwhGross>"));
    assert!(content.contains("<packCapacityKwhNet>75</packCapacityKwhNet>"));
    assert!(content.contains("<chemistry>NMC</chemistry>"));
}

#[test]
fn test_xml_charge_ports_section() {
    let vehicles = vec![create_test_vehicle()];
    let file = NamedTempFile::new().expect("Failed to create temp file");
    let path = file.path();

    xml::generate(&vehicles, path).expect("Failed to generate XML");

    let content = std::fs::read_to_string(path).expect("Failed to read generated file");

    assert!(content.contains("<chargePorts>"));
    assert!(content.contains("</chargePorts>"));
    assert!(content.contains("<port"));
    assert!(content.contains("Combo"));
    assert!(content.contains("Ccs2"));
}

#[test]
fn test_xml_charging_section() {
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

    xml::generate(&vehicles, path).expect("Failed to generate XML");

    let content = std::fs::read_to_string(path).expect("Failed to read generated file");

    assert!(content.contains("<charging>"));
    assert!(content.contains("</charging>"));
    assert!(content.contains("<dcMaxPowerKw>250</dcMaxPowerKw>"));
    assert!(content.contains("<acMaxPowerKw>11</acMaxPowerKw>"));
}

#[test]
fn test_xml_range_section() {
    let vehicles = vec![create_test_vehicle()];
    let file = NamedTempFile::new().expect("Failed to create temp file");
    let path = file.path();

    xml::generate(&vehicles, path).expect("Failed to generate XML");

    let content = std::fs::read_to_string(path).expect("Failed to read generated file");

    assert!(content.contains("<range>"));
    assert!(content.contains("</range>"));
    assert!(content.contains("<rated"));
    assert!(content.contains("Wltp"));
    assert!(content.contains(r#"km="500""#));
}

#[test]
fn test_xml_sources_section() {
    let vehicles = vec![create_test_vehicle()];
    let file = NamedTempFile::new().expect("Failed to create temp file");
    let path = file.path();

    xml::generate(&vehicles, path).expect("Failed to generate XML");

    let content = std::fs::read_to_string(path).expect("Failed to read generated file");

    assert!(content.contains("<sources>"));
    assert!(content.contains("</sources>"));
    assert!(content.contains("<source"));
    assert!(content.contains("Oem"));
    assert!(content.contains("https://tesla.com"));
}

#[test]
fn test_xml_escaping_ampersand() {
    let mut vehicle = create_test_vehicle();
    vehicle.make.name = "Tesla & Motors".to_string();

    let vehicles = vec![vehicle];
    let file = NamedTempFile::new().expect("Failed to create temp file");
    let path = file.path();

    xml::generate(&vehicles, path).expect("Failed to generate XML");

    let content = std::fs::read_to_string(path).expect("Failed to read generated file");

    assert!(content.contains("Tesla &amp; Motors"));
    assert!(!content.contains("Tesla & Motors</"));
}

#[test]
fn test_xml_escaping_less_than() {
    let mut vehicle = create_test_vehicle();
    vehicle.model.name = "Model <3".to_string();

    let vehicles = vec![vehicle];
    let file = NamedTempFile::new().expect("Failed to create temp file");
    let path = file.path();

    xml::generate(&vehicles, path).expect("Failed to generate XML");

    let content = std::fs::read_to_string(path).expect("Failed to read generated file");

    assert!(content.contains("Model &lt;3"));
}

#[test]
fn test_xml_escaping_greater_than() {
    let mut vehicle = create_test_vehicle();
    vehicle.trim.name = "Sport > Base".to_string();

    let vehicles = vec![vehicle];
    let file = NamedTempFile::new().expect("Failed to create temp file");
    let path = file.path();

    xml::generate(&vehicles, path).expect("Failed to generate XML");

    let content = std::fs::read_to_string(path).expect("Failed to read generated file");

    assert!(content.contains("Sport &gt; Base"));
}

#[test]
fn test_xml_escaping_quotes() {
    let mut vehicle = create_test_vehicle();
    vehicle.make.slug = r#"test"quote"#.to_string();

    let vehicles = vec![vehicle];
    let file = NamedTempFile::new().expect("Failed to create temp file");
    let path = file.path();

    xml::generate(&vehicles, path).expect("Failed to generate XML");

    let content = std::fs::read_to_string(path).expect("Failed to read generated file");

    assert!(content.contains("&quot;"));
}

#[test]
fn test_xml_escaping_apostrophe() {
    let mut vehicle = create_test_vehicle();
    vehicle.model.name = "Model's Best".to_string();

    let vehicles = vec![vehicle];
    let file = NamedTempFile::new().expect("Failed to create temp file");
    let path = file.path();

    xml::generate(&vehicles, path).expect("Failed to generate XML");

    let content = std::fs::read_to_string(path).expect("Failed to read generated file");

    assert!(content.contains("Model&apos;s Best"));
}

#[test]
fn test_xml_namespace() {
    let vehicles = vec![create_test_vehicle()];
    let file = NamedTempFile::new().expect("Failed to create temp file");
    let path = file.path();

    xml::generate(&vehicles, path).expect("Failed to generate XML");

    let content = std::fs::read_to_string(path).expect("Failed to read generated file");
    assert!(content.contains("xmlns=\"https://openevdata.org/schema/1.0\""));
}

#[test]
fn test_xml_charging_data() {
    let mut vehicle = create_test_vehicle();
    vehicle.charging.dc = Some(ev_core::ChargingDc {
        max_power_kw: 250.0,
        voltage_range_v: None,
        max_current_a: None,
        architecture_voltage_class: None,
        power_limits_by_voltage: None,
        notes: None,
    });
    vehicle.charging.ac = Some(ev_core::ChargingAc {
        max_power_kw: 11.0,
        supported_power_steps_kw: None,
        phases: None,
        voltage_range_v: None,
        frequency_hz: None,
        max_current_a: None,
        onboard_charger_count: None,
        notes: None,
    });

    let vehicles = vec![vehicle];
    let file = NamedTempFile::new().expect("Failed to create temp file");
    let path = file.path();

    xml::generate(&vehicles, path).expect("Failed to generate XML");

    let content = std::fs::read_to_string(path).expect("Failed to read generated file");
    assert!(content.contains("<dcMaxPowerKw>250</dcMaxPowerKw>"));
    assert!(content.contains("<acMaxPowerKw>11</acMaxPowerKw>"));
}

#[test]
fn test_xml_vehicle_type() {
    let vehicles = vec![create_test_vehicle()];
    let file = NamedTempFile::new().expect("Failed to create temp file");
    let path = file.path();

    xml::generate(&vehicles, path).expect("Failed to generate XML");

    let content = std::fs::read_to_string(path).expect("Failed to read generated file");

    assert!(content.contains("<vehicleType>"));
    assert!(content.contains("</vehicleType>"));
    assert!(content.contains("PassengerCar"));
}

#[test]
fn test_xml_minimal_vehicle() {
    let vehicles = vec![create_minimal_vehicle()];
    let file = NamedTempFile::new().expect("Failed to create temp file");
    let path = file.path();

    xml::generate(&vehicles, path).expect("Failed to generate XML");

    let content = std::fs::read_to_string(path).expect("Failed to read generated file");

    assert!(content.contains("<vehicle"));
    assert!(content.contains("<make"));
    assert!(content.contains("BMW"));
    assert!(content.contains("<chargePorts>"));
    assert!(content.contains("</chargePorts>"));
}
