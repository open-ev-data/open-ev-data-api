use ev_core::{Battery, Validate, ValidationError};

#[test]
fn test_battery_validation_success() {
    let battery = Battery {
        pack_capacity_kwh_net: Some(60.0),
        pack_capacity_kwh_gross: Some(62.0),
        ..Default::default()
    };
    assert!(battery.validate().is_ok());
}

#[test]
fn test_battery_validation_missing_capacity() {
    let battery = Battery {
        pack_capacity_kwh_net: None,
        pack_capacity_kwh_gross: None,
        ..Default::default()
    };
    let result = battery.validate();
    assert!(result.is_err());
    // Assert specific error matching ValidationError::MissingBatteryCapacity
    let error = result.unwrap_err();
    assert!(matches!(error, ValidationError::MissingBatteryCapacity));
}

#[test]
fn test_battery_capacity_helpers() {
    let b1 = Battery {
        pack_capacity_kwh_net: Some(60.0),
        pack_capacity_kwh_gross: None,
        ..Default::default()
    };
    assert_eq!(b1.usable_capacity_kwh(), Some(60.0));

    let b2 = Battery {
        pack_capacity_kwh_net: None,
        pack_capacity_kwh_gross: Some(75.0),
        ..Default::default()
    };
    assert_eq!(b2.usable_capacity_kwh(), Some(75.0));
}
