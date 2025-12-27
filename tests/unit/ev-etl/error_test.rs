use ev_etl::error::EtlError;

#[test]
fn test_etl_error_display() {
    let err = EtlError::FileRead {
        path: "test.json".to_string(),
    };
    assert_eq!(err.to_string(), "Failed to read file: test.json");

    let err = EtlError::Validation {
        vehicle_id: "test:id".to_string(),
        message: "missing field".to_string(),
    };
    assert_eq!(
        err.to_string(),
        "Validation failed for test:id: missing field"
    );
}
