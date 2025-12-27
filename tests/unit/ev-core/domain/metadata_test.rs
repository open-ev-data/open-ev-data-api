use ev_core::domain::enums::AvailabilityStatus;
use ev_core::{Metadata, Msrp, Pricing, VehicleAvailability};

#[test]
fn test_metadata_serialization() {
    let metadata = Metadata {
        created_at: Some("2024-01-01".to_string()),
        ..Default::default()
    };
    let json = serde_json::to_value(&metadata).unwrap();
    assert_eq!(json["created_at"], "2024-01-01");
}

#[test]
fn test_pricing_serialization() {
    let pricing = Pricing {
        msrp: Some(vec![Msrp {
            currency: "USD".to_string(),
            amount: 39990.0,
            country: Some("US".to_string()),
            year: Some(2024),
            notes: None,
        }]),
    };
    let json = serde_json::to_value(&pricing).unwrap();
    assert_eq!(json["msrp"][0]["currency"], "USD");
}

#[test]
fn test_availability_serialization() {
    let avail = VehicleAvailability {
        status: AvailabilityStatus::Production,
        start_year: Some(2023),
        end_year: None,
        notes: None,
    };
    let json = serde_json::to_value(&avail).unwrap();
    assert_eq!(json["status"], "production");
}
