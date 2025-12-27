use ev_core::{ChargePort, Charging, ChargingAc, ConnectorType, PortKind};

#[test]
fn test_charge_port_serialization() {
    let port = ChargePort {
        kind: PortKind::Combo,
        connector: ConnectorType::Ccs2,
        location: None,
        covers: None,
        light: None,
        motorized: None,
        notes: None,
    };

    let json = serde_json::to_value(&port).expect("failed to serialize");
    assert_eq!(json["kind"], "combo");
    assert_eq!(json["connector"], "ccs2");
}

#[test]
fn test_charging_defaults() {
    let charging = Charging::default();
    assert!(charging.ac.is_none());
    assert!(charging.dc.is_none());
}

#[test]
fn test_charging_ac() {
    let ac = ChargingAc {
        max_power_kw: 11.0,
        supported_power_steps_kw: None,
        phases: Some(3),
        voltage_range_v: None,
        frequency_hz: None,
        max_current_a: None,
        onboard_charger_count: None,
        notes: None,
    };

    assert_eq!(ac.max_power_kw, 11.0);
    assert_eq!(ac.phases, Some(3));
}
