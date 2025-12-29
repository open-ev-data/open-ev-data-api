use ev_etl::ingest::parser::{parse_json_file, validate_json_object};
use serde_json::json;
use std::io::Write;
use std::path::Path;
use tempfile::NamedTempFile;

#[test]
fn test_validate_json_object_valid() {
    let obj = json!({"key": "value"});
    assert!(validate_json_object(&obj, Path::new("test.json")).is_ok());
}

#[test]
fn test_validate_json_object_invalid() {
    let cases = vec![
        (json!([]), "array"),
        (json!(null), "null"),
        (json!(true), "boolean"),
        (json!(42), "number"),
        (json!("string"), "string"),
    ];

    for (value, type_name) in cases {
        let err = validate_json_object(&value, Path::new("test.json")).unwrap_err();
        assert!(
            err.to_string().contains(&format!("got {}", type_name)),
            "Expected error to contain 'got {}', found: {}",
            type_name,
            err
        );
    }
}

#[test]
fn test_parse_json_file_success() {
    let mut file = NamedTempFile::new().unwrap();
    write!(file, r#"{{"key": "value"}}"#).unwrap();

    let path = file.path();
    let result = parse_json_file(path).unwrap();

    assert_eq!(result["key"], "value");
}

#[test]
fn test_parse_json_file_not_found() {
    let path = Path::new("non_existent_file.json");
    assert!(parse_json_file(path).is_err());
}

#[test]
fn test_parse_json_file_invalid_json() {
    let mut file = NamedTempFile::new().unwrap();
    write!(file, "invalid json").unwrap();

    assert!(parse_json_file(file.path()).is_err());
}
