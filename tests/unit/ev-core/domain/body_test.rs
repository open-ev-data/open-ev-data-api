use ev_core::{Body, Dimensions, Weights};

#[test]
fn test_body_serialization() {
    let body = Body {
        style: Some("sedan".to_string()),
        doors: Some(4),
        seats: Some(5),
        ..Default::default()
    };

    let json = serde_json::to_value(&body).expect("failed to serialize");
    assert_eq!(json["style"], "sedan");
    assert_eq!(json["doors"], 4);
    assert_eq!(json["seats"], 5);
    assert!(json["rows"].is_null());
}

#[test]
fn test_dimensions_default() {
    let dim = Dimensions::default();
    assert!(dim.length_mm.is_none());
    assert!(dim.width_mm.is_none());
}

#[test]
fn test_weights_serialization() {
    let weights = Weights {
        curb_weight_kg: Some(2000.0),
        gross_vehicle_weight_kg: Some(2500.0),
        ..Default::default()
    };
    let json = serde_json::to_value(&weights).expect("failed to serialize");
    assert_eq!(json["curb_weight_kg"], 2000.0);
}
