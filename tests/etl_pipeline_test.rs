use std::path::PathBuf;

use tempfile::TempDir;

fn fixtures_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures")
        .join("sample_vehicles")
}

#[test]
fn test_ingest_sample_vehicles() {
    let input_dir = fixtures_dir();
    if !input_dir.exists() {
        eprintln!("Fixtures directory not found, skipping test");
        return;
    }

    let result = ev_etl_test_ingest(&input_dir);
    assert!(result.is_ok(), "Ingest failed: {:?}", result.err());
}

#[test]
fn test_etl_pipeline_json_output() {
    let input_dir = fixtures_dir();
    if !input_dir.exists() {
        eprintln!("Fixtures directory not found, skipping test");
        return;
    }

    let output_dir = TempDir::new().expect("Failed to create temp dir");

    let result = run_etl_pipeline(&input_dir, output_dir.path(), &["json"]);
    assert!(result.is_ok(), "ETL failed: {:?}", result.err());

    let json_path = output_dir.path().join("vehicles.json");
    assert!(json_path.exists(), "JSON output not created");

    let content = std::fs::read_to_string(&json_path).expect("Failed to read JSON");
    let parsed: serde_json::Value = serde_json::from_str(&content).expect("Invalid JSON");

    assert!(parsed["vehicles"].is_array());
    assert!(parsed["vehicle_count"].as_u64().unwrap() > 0);
}

#[test]
fn test_etl_pipeline_sqlite_output() {
    let input_dir = fixtures_dir();
    if !input_dir.exists() {
        eprintln!("Fixtures directory not found, skipping test");
        return;
    }

    let output_dir = TempDir::new().expect("Failed to create temp dir");

    let result = run_etl_pipeline(&input_dir, output_dir.path(), &["sqlite"]);
    assert!(result.is_ok(), "ETL failed: {:?}", result.err());

    let db_path = output_dir.path().join("vehicles.db");
    assert!(db_path.exists(), "SQLite database not created");

    let conn = rusqlite::Connection::open(&db_path).expect("Failed to open database");
    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM vehicles", [], |row| row.get(0))
        .expect("Failed to query");

    assert!(count > 0, "No vehicles in database");
}

fn ev_etl_test_ingest(input_dir: &std::path::Path) -> anyhow::Result<Vec<serde_json::Value>> {
    use walkdir::WalkDir;

    let mut vehicles = Vec::new();

    for entry in WalkDir::new(input_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "json"))
    {
        let content = std::fs::read_to_string(entry.path())?;
        let value: serde_json::Value = serde_json::from_str(&content)?;
        vehicles.push(value);
    }

    Ok(vehicles)
}

fn run_etl_pipeline(
    input_dir: &std::path::Path,
    output_dir: &std::path::Path,
    formats: &[&str],
) -> anyhow::Result<()> {
    use std::process::Command;

    let status = Command::new(env!("CARGO"))
        .args([
            "run",
            "-p",
            "ev-etl",
            "--",
            "--input",
            input_dir.to_str().unwrap(),
            "--output",
            output_dir.to_str().unwrap(),
            "--formats",
            &formats.join(","),
        ])
        .status()?;

    if status.success() {
        Ok(())
    } else {
        anyhow::bail!("ETL command failed")
    }
}
