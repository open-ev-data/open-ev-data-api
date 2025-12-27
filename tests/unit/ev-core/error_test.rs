use ev_core::{CoreError, ValidationError};

#[test]
fn test_validation_error_display() {
    let err = ValidationError::missing_field("battery");
    assert_eq!(err.to_string(), "Missing required field: battery");

    let err = ValidationError::invalid_slug("Invalid Slug");
    assert_eq!(
        err.to_string(),
        "Invalid slug format: 'Invalid Slug'. Must be lowercase alphanumeric with underscores"
    );
}

#[test]
fn test_validation_error_helpers() {
    let err = ValidationError::empty_value("name");
    assert!(matches!(err, ValidationError::EmptyValue { field } if field == "name"));
}

#[test]
fn test_core_error_conversion() {
    let validation_err = ValidationError::missing_field("test");
    let core_err: CoreError = validation_err.into();

    match core_err {
        CoreError::Validation(e) => {
            assert_eq!(e.to_string(), "Missing required field: test");
        }
        _ => panic!("Expected Validation error variant"),
    }
}
