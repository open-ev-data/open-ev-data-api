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
fn test_battery_validation_with_gross_only() {
    let battery = Battery {
        pack_capacity_kwh_net: None,
        pack_capacity_kwh_gross: Some(65.0),
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
    let error = result.unwrap_err();
    assert!(matches!(error, ValidationError::MissingBatteryCapacity));
}

#[test]
fn test_battery_has_capacity_with_net() {
    let battery = Battery {
        pack_capacity_kwh_net: Some(60.0),
        pack_capacity_kwh_gross: None,
        ..Default::default()
    };
    assert!(battery.has_capacity());
}

#[test]
fn test_battery_has_capacity_with_gross() {
    let battery = Battery {
        pack_capacity_kwh_net: None,
        pack_capacity_kwh_gross: Some(65.0),
        ..Default::default()
    };
    assert!(battery.has_capacity());
}

#[test]
fn test_battery_has_capacity_with_both() {
    let battery = Battery {
        pack_capacity_kwh_net: Some(60.0),
        pack_capacity_kwh_gross: Some(65.0),
        ..Default::default()
    };
    assert!(battery.has_capacity());
}

#[test]
fn test_battery_has_capacity_none() {
    let battery = Battery {
        pack_capacity_kwh_net: None,
        pack_capacity_kwh_gross: None,
        ..Default::default()
    };
    assert!(!battery.has_capacity());
}

#[test]
fn test_battery_usable_capacity_prefers_net() {
    let battery = Battery {
        pack_capacity_kwh_net: Some(60.0),
        pack_capacity_kwh_gross: Some(65.0),
        ..Default::default()
    };
    assert_eq!(battery.usable_capacity_kwh(), Some(60.0));
}

#[test]
fn test_battery_usable_capacity_fallback_to_gross() {
    let battery = Battery {
        pack_capacity_kwh_net: None,
        pack_capacity_kwh_gross: Some(75.0),
        ..Default::default()
    };
    assert_eq!(battery.usable_capacity_kwh(), Some(75.0));
}

#[test]
fn test_battery_usable_capacity_none() {
    let battery = Battery::default();
    assert_eq!(battery.usable_capacity_kwh(), None);
}

#[test]
fn test_battery_serialization() {
    let battery = Battery {
        pack_capacity_kwh_net: Some(60.0),
        pack_capacity_kwh_gross: Some(65.0),
        chemistry: Some("NMC".to_string()),
        ..Default::default()
    };

    let json = serde_json::to_value(&battery).unwrap();
    assert_eq!(json["pack_capacity_kwh_net"], 60.0);
    assert_eq!(json["pack_capacity_kwh_gross"], 65.0);
    assert_eq!(json["chemistry"], "NMC");
}

#[test]
fn test_battery_default() {
    let battery = Battery::default();
    assert_eq!(battery.pack_capacity_kwh_net, None);
    assert_eq!(battery.pack_capacity_kwh_gross, None);
    assert_eq!(battery.chemistry, None);
    assert!(!battery.has_capacity());
}
