use ev_core::domain::enums::{
    AvailabilityStatus, ChargeCurveType, ChargerVoltageClass, ConnectorType, Drivetrain,
    MotorPosition, PortKind, PortLocation, PortPosition, RangeCycle, RealWorldProfile, SourceType,
    ThermalManagement, VehicleType,
};

#[test]
fn test_vehicle_type_display_passenger_car() {
    assert_eq!(VehicleType::PassengerCar.to_string(), "Passenger Car");
}

#[test]
fn test_vehicle_type_display_suv() {
    assert_eq!(VehicleType::Suv.to_string(), "SUV");
}

#[test]
fn test_vehicle_type_display_pickup() {
    assert_eq!(VehicleType::Pickup.to_string(), "Pickup");
}

#[test]
fn test_vehicle_type_display_van() {
    assert_eq!(VehicleType::Van.to_string(), "Van");
}

#[test]
fn test_vehicle_type_display_bus() {
    assert_eq!(VehicleType::Bus.to_string(), "Bus");
}

#[test]
fn test_vehicle_type_display_motorcycle() {
    assert_eq!(VehicleType::Motorcycle.to_string(), "Motorcycle");
}

#[test]
fn test_vehicle_type_display_scooter() {
    assert_eq!(VehicleType::Scooter.to_string(), "Scooter");
}

#[test]
fn test_vehicle_type_display_commercial() {
    assert_eq!(VehicleType::Commercial.to_string(), "Commercial");
}

#[test]
fn test_vehicle_type_display_truck() {
    assert_eq!(VehicleType::Truck.to_string(), "Truck");
}

#[test]
fn test_vehicle_type_display_other() {
    assert_eq!(VehicleType::Other.to_string(), "Other");
}

#[test]
fn test_drivetrain_display_fwd() {
    assert_eq!(Drivetrain::Fwd.to_string(), "FWD");
}

#[test]
fn test_drivetrain_display_rwd() {
    assert_eq!(Drivetrain::Rwd.to_string(), "RWD");
}

#[test]
fn test_drivetrain_display_awd() {
    assert_eq!(Drivetrain::Awd.to_string(), "AWD");
}

#[test]
fn test_drivetrain_display_four_wd() {
    assert_eq!(Drivetrain::FourWd.to_string(), "4WD");
}

#[test]
fn test_connector_type_display_type1() {
    assert_eq!(ConnectorType::Type1.to_string(), "Type 1");
}

#[test]
fn test_connector_type_display_type2() {
    assert_eq!(ConnectorType::Type2.to_string(), "Type 2");
}

#[test]
fn test_connector_type_display_ccs1() {
    assert_eq!(ConnectorType::Ccs1.to_string(), "CCS1");
}

#[test]
fn test_connector_type_display_ccs2() {
    assert_eq!(ConnectorType::Ccs2.to_string(), "CCS2");
}

#[test]
fn test_connector_type_display_nacs() {
    assert_eq!(ConnectorType::Nacs.to_string(), "NACS");
}

#[test]
fn test_connector_type_display_chademo() {
    assert_eq!(ConnectorType::Chademo.to_string(), "CHAdeMO");
}

#[test]
fn test_connector_type_display_gbt_ac() {
    assert_eq!(ConnectorType::GbTAc.to_string(), "GB/T AC");
}

#[test]
fn test_connector_type_display_gbt_dc() {
    assert_eq!(ConnectorType::GbTDc.to_string(), "GB/T DC");
}

#[test]
fn test_connector_type_display_tesla_type2() {
    assert_eq!(ConnectorType::TeslaType2.to_string(), "Tesla Type 2");
}

#[test]
fn test_connector_type_display_other() {
    assert_eq!(ConnectorType::Other.to_string(), "Other");
}

#[test]
fn test_range_cycle_display_wltp() {
    assert_eq!(RangeCycle::Wltp.to_string(), "WLTP");
}

#[test]
fn test_range_cycle_display_epa() {
    assert_eq!(RangeCycle::Epa.to_string(), "EPA");
}

#[test]
fn test_range_cycle_display_nedc() {
    assert_eq!(RangeCycle::Nedc.to_string(), "NEDC");
}

#[test]
fn test_range_cycle_display_cltc() {
    assert_eq!(RangeCycle::Cltc.to_string(), "CLTC");
}

#[test]
fn test_range_cycle_display_jc08() {
    assert_eq!(RangeCycle::Jc08.to_string(), "JC08");
}

#[test]
fn test_range_cycle_display_other() {
    assert_eq!(RangeCycle::Other.to_string(), "Other");
}

#[test]
fn test_vehicle_type_serialization() {
    let vehicle_type = VehicleType::PassengerCar;
    let json = serde_json::to_value(&vehicle_type).unwrap();
    assert_eq!(json, serde_json::json!("passenger_car"));
}

#[test]
fn test_vehicle_type_deserialization() {
    let json = serde_json::json!("suv");
    let vehicle_type: VehicleType = serde_json::from_value(json).unwrap();
    assert_eq!(vehicle_type, VehicleType::Suv);
}

#[test]
fn test_drivetrain_serialization() {
    let drivetrain = Drivetrain::FourWd;
    let json = serde_json::to_value(&drivetrain).unwrap();
    assert_eq!(json, serde_json::json!("4wd"));
}

#[test]
fn test_drivetrain_deserialization() {
    let json = serde_json::json!("awd");
    let drivetrain: Drivetrain = serde_json::from_value(json).unwrap();
    assert_eq!(drivetrain, Drivetrain::Awd);
}

#[test]
fn test_motor_position_serialization() {
    let pos = MotorPosition::Front;
    let json = serde_json::to_value(&pos).unwrap();
    assert_eq!(json, serde_json::json!("front"));

    let pos = MotorPosition::Rear;
    let json = serde_json::to_value(&pos).unwrap();
    assert_eq!(json, serde_json::json!("rear"));

    let pos = MotorPosition::Other;
    let json = serde_json::to_value(&pos).unwrap();
    assert_eq!(json, serde_json::json!("other"));
}

#[test]
fn test_thermal_management_serialization() {
    assert_eq!(
        serde_json::to_value(ThermalManagement::Liquid).unwrap(),
        serde_json::json!("liquid")
    );
    assert_eq!(
        serde_json::to_value(ThermalManagement::Air).unwrap(),
        serde_json::json!("air")
    );
    assert_eq!(
        serde_json::to_value(ThermalManagement::Passive).unwrap(),
        serde_json::json!("passive")
    );
    assert_eq!(
        serde_json::to_value(ThermalManagement::Refrigerant).unwrap(),
        serde_json::json!("refrigerant")
    );
    assert_eq!(
        serde_json::to_value(ThermalManagement::None).unwrap(),
        serde_json::json!("none")
    );
}

#[test]
fn test_port_kind_serialization() {
    assert_eq!(
        serde_json::to_value(PortKind::AcOnly).unwrap(),
        serde_json::json!("ac_only")
    );
    assert_eq!(
        serde_json::to_value(PortKind::DcOnly).unwrap(),
        serde_json::json!("dc_only")
    );
    assert_eq!(
        serde_json::to_value(PortKind::Combo).unwrap(),
        serde_json::json!("combo")
    );
}

#[test]
fn test_connector_type_serialization() {
    assert_eq!(
        serde_json::to_value(ConnectorType::Type1).unwrap(),
        serde_json::json!("type1")
    );
    assert_eq!(
        serde_json::to_value(ConnectorType::Ccs2).unwrap(),
        serde_json::json!("ccs2")
    );
    assert_eq!(
        serde_json::to_value(ConnectorType::Nacs).unwrap(),
        serde_json::json!("nacs")
    );
}

#[test]
fn test_port_location_serialization() {
    assert_eq!(
        serde_json::to_value(PortLocation::Left).unwrap(),
        serde_json::json!("left")
    );
    assert_eq!(
        serde_json::to_value(PortLocation::Right).unwrap(),
        serde_json::json!("right")
    );
    assert_eq!(
        serde_json::to_value(PortLocation::Front).unwrap(),
        serde_json::json!("front")
    );
    assert_eq!(
        serde_json::to_value(PortLocation::Rear).unwrap(),
        serde_json::json!("rear")
    );
    assert_eq!(
        serde_json::to_value(PortLocation::Center).unwrap(),
        serde_json::json!("center")
    );
}

#[test]
fn test_port_position_serialization() {
    assert_eq!(
        serde_json::to_value(PortPosition::Front).unwrap(),
        serde_json::json!("front")
    );
    assert_eq!(
        serde_json::to_value(PortPosition::Rear).unwrap(),
        serde_json::json!("rear")
    );
    assert_eq!(
        serde_json::to_value(PortPosition::Mid).unwrap(),
        serde_json::json!("mid")
    );
}

#[test]
fn test_charger_voltage_class_serialization() {
    assert_eq!(
        serde_json::to_value(ChargerVoltageClass::V400).unwrap(),
        serde_json::json!("400v")
    );
    assert_eq!(
        serde_json::to_value(ChargerVoltageClass::V800).unwrap(),
        serde_json::json!("800v")
    );
    assert_eq!(
        serde_json::to_value(ChargerVoltageClass::Other).unwrap(),
        serde_json::json!("other")
    );
}

#[test]
fn test_charge_curve_type_serialization() {
    assert_eq!(
        serde_json::to_value(ChargeCurveType::PowerBySoc).unwrap(),
        serde_json::json!("power_by_soc")
    );
    assert_eq!(
        serde_json::to_value(ChargeCurveType::CurrentBySoc).unwrap(),
        serde_json::json!("current_by_soc")
    );
}

#[test]
fn test_range_cycle_serialization() {
    assert_eq!(
        serde_json::to_value(RangeCycle::Wltp).unwrap(),
        serde_json::json!("wltp")
    );
    assert_eq!(
        serde_json::to_value(RangeCycle::Epa).unwrap(),
        serde_json::json!("epa")
    );
    assert_eq!(
        serde_json::to_value(RangeCycle::Nedc).unwrap(),
        serde_json::json!("nedc")
    );
    assert_eq!(
        serde_json::to_value(RangeCycle::Cltc).unwrap(),
        serde_json::json!("cltc")
    );
    assert_eq!(
        serde_json::to_value(RangeCycle::Jc08).unwrap(),
        serde_json::json!("jc08")
    );
    assert_eq!(
        serde_json::to_value(RangeCycle::Other).unwrap(),
        serde_json::json!("other")
    );
}

#[test]
fn test_real_world_profile_serialization() {
    assert_eq!(
        serde_json::to_value(RealWorldProfile::Highway).unwrap(),
        serde_json::json!("highway")
    );
    assert_eq!(
        serde_json::to_value(RealWorldProfile::City).unwrap(),
        serde_json::json!("city")
    );
    assert_eq!(
        serde_json::to_value(RealWorldProfile::Mixed).unwrap(),
        serde_json::json!("mixed")
    );
    assert_eq!(
        serde_json::to_value(RealWorldProfile::ColdWeather).unwrap(),
        serde_json::json!("cold_weather")
    );
    assert_eq!(
        serde_json::to_value(RealWorldProfile::Winter).unwrap(),
        serde_json::json!("winter")
    );
    assert_eq!(
        serde_json::to_value(RealWorldProfile::Summer).unwrap(),
        serde_json::json!("summer")
    );
}

#[test]
fn test_availability_status_serialization() {
    assert_eq!(
        serde_json::to_value(AvailabilityStatus::Production).unwrap(),
        serde_json::json!("production")
    );
    assert_eq!(
        serde_json::to_value(AvailabilityStatus::Discontinued).unwrap(),
        serde_json::json!("discontinued")
    );
    assert_eq!(
        serde_json::to_value(AvailabilityStatus::Concept).unwrap(),
        serde_json::json!("concept")
    );
    assert_eq!(
        serde_json::to_value(AvailabilityStatus::Announced).unwrap(),
        serde_json::json!("announced")
    );
    assert_eq!(
        serde_json::to_value(AvailabilityStatus::Prototype).unwrap(),
        serde_json::json!("prototype")
    );
}

#[test]
fn test_source_type_serialization() {
    assert_eq!(
        serde_json::to_value(SourceType::Oem).unwrap(),
        serde_json::json!("oem")
    );
    assert_eq!(
        serde_json::to_value(SourceType::Regulatory).unwrap(),
        serde_json::json!("regulatory")
    );
    assert_eq!(
        serde_json::to_value(SourceType::Press).unwrap(),
        serde_json::json!("press")
    );
    assert_eq!(
        serde_json::to_value(SourceType::Community).unwrap(),
        serde_json::json!("community")
    );
    assert_eq!(
        serde_json::to_value(SourceType::TestingOrg).unwrap(),
        serde_json::json!("testing_org")
    );
}

#[test]
fn test_enum_equality() {
    assert_eq!(VehicleType::Suv, VehicleType::Suv);
    assert_ne!(VehicleType::Suv, VehicleType::Pickup);
    assert_eq!(Drivetrain::Awd, Drivetrain::Awd);
    assert_ne!(Drivetrain::Fwd, Drivetrain::Rwd);
}

#[test]
fn test_enum_clone() {
    let original = VehicleType::PassengerCar;
    let cloned = original.clone();
    assert_eq!(original, cloned);

    let original = ConnectorType::Ccs2;
    let cloned = original.clone();
    assert_eq!(original, cloned);
}

#[test]
fn test_enum_debug() {
    assert_eq!(format!("{:?}", VehicleType::PassengerCar), "PassengerCar");
    assert_eq!(format!("{:?}", Drivetrain::Awd), "Awd");
    assert_eq!(format!("{:?}", ConnectorType::Ccs2), "Ccs2");
}
