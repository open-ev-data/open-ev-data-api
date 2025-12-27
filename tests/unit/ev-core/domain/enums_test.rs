use ev_core::domain::enums::{
    ConnectorType, Drivetrain, RangeCycle, ThermalManagement, VehicleType,
};

#[test]
fn test_vehicle_type_display() {
    assert_eq!(VehicleType::PassengerCar.to_string(), "Passenger Car");
    assert_eq!(VehicleType::Suv.to_string(), "SUV");
}

#[test]
fn test_drivetrain_display() {
    assert_eq!(Drivetrain::Awd.to_string(), "AWD");
    assert_eq!(Drivetrain::Fwd.to_string(), "FWD");
}

#[test]
fn test_connector_type_display() {
    assert_eq!(ConnectorType::Ccs2.to_string(), "CCS2");
    assert_eq!(ConnectorType::Nacs.to_string(), "NACS");
}

#[test]
fn test_range_cycle_display() {
    assert_eq!(RangeCycle::Wltp.to_string(), "WLTP");
}

#[test]
fn test_enum_serialization() {
    let mode = ThermalManagement::Liquid;
    let json = serde_json::to_value(&mode).unwrap();
    assert_eq!(json, serde_json::json!("liquid"));
}
