use ev_etl::ingest::reader::{FileType, scan_directory};
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

#[test]
fn test_scan_directory_hidden_files() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    let model_dir = root.join("tesla").join("model_3");
    let year_dir = model_dir.join("2024");
    fs::create_dir_all(&year_dir).unwrap();

    // Hidden file should be ignored
    fs::write(year_dir.join(".hidden.json"), "{}").unwrap();
    // Valid file
    fs::write(year_dir.join("tesla_model_3.json"), "{}").unwrap();

    let files = scan_directory(root).unwrap();
    assert_eq!(files.len(), 1);
    assert_eq!(files[0].file_type, FileType::YearBase);
}

#[test]
fn test_scan_directory_invalid_year() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    let model_dir = root.join("tesla").join("model_3");
    let year_dir = model_dir.join("invalid_year");
    fs::create_dir_all(&year_dir).unwrap();
    fs::write(year_dir.join("tesla_model_3.json"), "{}").unwrap();

    let files = scan_directory(root).unwrap();
    // Invalid year folder should be skipped
    assert!(files.is_empty());
}

#[test]
fn test_scan_directory_year_base_without_underscore() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    let model_dir = root.join("byd").join("dolphin");
    let year_dir = model_dir.join("2024");
    fs::create_dir_all(&year_dir).unwrap();
    // File without underscore pattern should still be YearBase
    fs::write(year_dir.join("base.json"), "{}").unwrap();

    let files = scan_directory(root).unwrap();
    assert_eq!(files.len(), 1);
    assert_eq!(files[0].file_type, FileType::YearBase);
}

#[test]
fn test_scan_directory_sorting() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    let model_dir = root.join("tesla").join("model_3");
    let year_dir = model_dir.join("2024");
    fs::create_dir_all(&year_dir).unwrap();
    fs::write(model_dir.join("base.json"), "{}").unwrap();
    fs::write(year_dir.join("tesla_model_3.json"), "{}").unwrap();
    fs::write(year_dir.join("tesla_model_3_lr.json"), "{}").unwrap();

    let files = scan_directory(root).unwrap();
    assert_eq!(files.len(), 3);

    // Should be sorted: ModelBase, YearBase, Variant
    assert_eq!(files[0].file_type, FileType::ModelBase);
    assert_eq!(files[1].file_type, FileType::YearBase);
    assert_eq!(files[2].file_type, FileType::Variant);
}

#[test]
fn test_scan_directory_non_json_files() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    let model_dir = root.join("tesla").join("model_3");
    let year_dir = model_dir.join("2024");
    fs::create_dir_all(&year_dir).unwrap();
    fs::write(year_dir.join("readme.txt"), "text").unwrap();
    fs::write(year_dir.join("config.yaml"), "yaml").unwrap();
    fs::write(year_dir.join("tesla_model_3.json"), "{}").unwrap();

    let files = scan_directory(root).unwrap();
    assert_eq!(files.len(), 1);
}

#[test]
fn test_scan_directory_empty() {
    let dir = tempdir().unwrap();
    let files = scan_directory(dir.path()).unwrap();
    assert!(files.is_empty());
}

#[test]
fn test_scan_directory_multiple_makes() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // Tesla
    let tesla_dir = root.join("tesla").join("model_3").join("2024");
    fs::create_dir_all(&tesla_dir).unwrap();
    fs::write(tesla_dir.join("tesla_model_3.json"), "{}").unwrap();

    // BYD
    let byd_dir = root.join("byd").join("dolphin").join("2024");
    fs::create_dir_all(&byd_dir).unwrap();
    fs::write(byd_dir.join("byd_dolphin.json"), "{}").unwrap();

    let files = scan_directory(root).unwrap();
    assert_eq!(files.len(), 2);

    // Should be sorted by make
    assert_eq!(files[0].make_slug, "byd");
    assert_eq!(files[1].make_slug, "tesla");
}

#[test]
fn test_scan_directory_variant_with_underscore() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    let model_dir = root.join("tesla").join("model_3");
    let year_dir = model_dir.join("2024");
    fs::create_dir_all(&year_dir).unwrap();

    // YearBase file: tesla_model_3.json
    fs::write(year_dir.join("tesla_model_3.json"), "{}").unwrap();
    // Variant file with underscore suffix: tesla_model_3_long_range.json
    fs::write(year_dir.join("tesla_model_3_long_range.json"), "{}").unwrap();
    // Another variant: a file with underscore but not matching expected pattern exactly
    fs::write(year_dir.join("some_other_variant.json"), "{}").unwrap();

    let files = scan_directory(root).unwrap();

    // Should have YearBase + 2 Variants
    assert_eq!(files.len(), 3);

    let year_bases: Vec<_> = files
        .iter()
        .filter(|f| f.file_type == FileType::YearBase)
        .collect();
    let variants: Vec<_> = files
        .iter()
        .filter(|f| f.file_type == FileType::Variant)
        .collect();

    assert_eq!(year_bases.len(), 1);
    assert_eq!(variants.len(), 2);
}
