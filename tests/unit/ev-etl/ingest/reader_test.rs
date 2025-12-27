use ev_etl::ingest::reader::{scan_directory, FileType};
use std::fs;
use tempfile::tempdir;

#[test]
fn test_scan_directory_structure() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // Create structure: tesla/model_y/base.json
    let model_dir = root.join("tesla").join("model_y");
    fs::create_dir_all(&model_dir).unwrap();
    fs::write(model_dir.join("base.json"), "{}").unwrap();

    // Create structure: tesla/model_y/2024/tesla_model_y.json (YearBase)
    let year_dir = model_dir.join("2024");
    fs::create_dir_all(&year_dir).unwrap();
    fs::write(year_dir.join("tesla_model_y.json"), "{}").unwrap();

    // Create structure: tesla/model_y/2024/tesla_model_y_perf.json (Variant)
    fs::write(year_dir.join("tesla_model_y_perf.json"), "{}").unwrap();

    let files = scan_directory(root).unwrap();

    assert_eq!(files.len(), 3);

    // Check ModelBase
    let base = files
        .iter()
        .find(|f| f.file_type == FileType::ModelBase)
        .unwrap();
    assert_eq!(base.make_slug, "tesla");
    assert_eq!(base.model_slug, "model_y");
    assert_eq!(base.year, None);

    // Check YearBase
    let year_base = files
        .iter()
        .find(|f| f.file_type == FileType::YearBase)
        .unwrap();
    assert_eq!(year_base.year, Some(2024));

    // Check Variant
    let variant = files
        .iter()
        .find(|f| f.file_type == FileType::Variant)
        .unwrap();
    assert_eq!(variant.year, Some(2024));
}

#[test]
fn test_scan_directory_invalid_structure() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // Shallow file, ignored (requires make/model depth)
    fs::write(root.join("shallow.json"), "{}").unwrap();

    let files = scan_directory(root).unwrap();
    assert!(files.is_empty());
}
