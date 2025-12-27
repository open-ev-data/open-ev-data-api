use ev_core::{Source, SourceType};

#[test]
fn test_source_is_official_oem() {
    let source = Source {
        source_type: SourceType::Oem,
        title: "Official Specs".to_string(),
        url: "https://example.com".to_string(),
        accessed_at: "2024-01-01".to_string(),
        publisher: None,
        license: None,
        notes: None,
    };

    assert!(source.is_official());
}

#[test]
fn test_source_is_official_regulatory() {
    let source = Source {
        source_type: SourceType::Regulatory,
        title: "EPA Data".to_string(),
        url: "https://epa.gov".to_string(),
        accessed_at: "2024-01-01".to_string(),
        publisher: Some("EPA".to_string()),
        license: None,
        notes: None,
    };

    assert!(source.is_official());
}

#[test]
fn test_source_is_not_official_press() {
    let source = Source {
        source_type: SourceType::Press,
        title: "News Article".to_string(),
        url: "https://news.com".to_string(),
        accessed_at: "2024-01-01".to_string(),
        publisher: Some("News Corp".to_string()),
        license: None,
        notes: None,
    };

    assert!(!source.is_official());
}

#[test]
fn test_source_is_not_official_community() {
    let source = Source {
        source_type: SourceType::Community,
        title: "Forum Post".to_string(),
        url: "https://forum.com".to_string(),
        accessed_at: "2024-01-01".to_string(),
        publisher: None,
        license: None,
        notes: Some("User submitted".to_string()),
    };

    assert!(!source.is_official());
}

#[test]
fn test_source_is_not_official_testing_org() {
    let source = Source {
        source_type: SourceType::TestingOrg,
        title: "Test Results".to_string(),
        url: "https://testing.org".to_string(),
        accessed_at: "2024-01-01".to_string(),
        publisher: Some("Test Organization".to_string()),
        license: Some("CC-BY-4.0".to_string()),
        notes: None,
    };

    assert!(!source.is_official());
}

#[test]
fn test_source_serialization() {
    let source = Source {
        source_type: SourceType::Oem,
        title: "Official Specs".to_string(),
        url: "https://example.com".to_string(),
        accessed_at: "2024-01-01".to_string(),
        publisher: None,
        license: None,
        notes: None,
    };

    let json = serde_json::to_value(&source).unwrap();
    assert_eq!(json["type"], "oem");
    assert_eq!(json["url"], "https://example.com");
    assert_eq!(json["title"], "Official Specs");
}

#[test]
fn test_source_deserialization() {
    let json = r#"{
        "type": "regulatory",
        "title": "EPA Data",
        "url": "https://epa.gov",
        "accessed_at": "2024-01-01",
        "publisher": "EPA"
    }"#;

    let source: Source = serde_json::from_str(json).unwrap();
    assert_eq!(source.source_type, SourceType::Regulatory);
    assert_eq!(source.publisher, Some("EPA".to_string()));
    assert!(source.is_official());
}

#[test]
fn test_source_with_all_optional_fields() {
    let source = Source {
        source_type: SourceType::Press,
        title: "Full Article".to_string(),
        url: "https://example.com/article".to_string(),
        accessed_at: "2024-06-15".to_string(),
        publisher: Some("Publisher Inc.".to_string()),
        license: Some("MIT".to_string()),
        notes: Some("Detailed notes".to_string()),
    };

    let json = serde_json::to_value(&source).unwrap();
    assert_eq!(json["publisher"], "Publisher Inc.");
    assert_eq!(json["license"], "MIT");
    assert_eq!(json["notes"], "Detailed notes");
}
