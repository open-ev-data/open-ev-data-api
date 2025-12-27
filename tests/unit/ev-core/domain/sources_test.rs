use ev_core::{Source, SourceType};

#[test]
fn test_source_validation() {
    // Basic structural test, assuming validation logic might be in Validator
    let source = Source {
        source_type: SourceType::Oem,
        title: "Official Specs".to_string(),
        url: "https://example.com".to_string(),
        accessed_at: "2024-01-01".to_string(),
        publisher: None,
        license: None,
        notes: None,
    };

    // Check if serialization works
    let json = serde_json::to_value(&source).unwrap();
    assert_eq!(json["url"], "https://example.com");
}
