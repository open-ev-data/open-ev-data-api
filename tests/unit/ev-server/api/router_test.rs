use ev_server::api::create_router;
use ev_server::config::Config;
use ev_server::db::Database;
use std::sync::Arc;
use tempfile::TempDir;

fn create_test_db() -> (TempDir, Arc<Database>) {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let db_path = temp_dir.path().join("test.db");
    let db = Database::new(db_path.to_str().unwrap()).expect("Failed to create db");
    (temp_dir, Arc::new(db))
}

fn create_config(enable_openapi: bool, enable_compression: bool) -> Config {
    Config {
        database_url: "test.db".to_string(),
        port: 3000,
        host: "0.0.0.0".to_string(),
        log_level: "info".to_string(),
        cors_origins: vec!["*".to_string()],
        max_page_size: 100,
        enable_compression,
        enable_openapi,
    }
}

#[test]
fn test_create_router_with_defaults() {
    let (temp_dir, db) = create_test_db();
    let config = create_config(true, true);

    let router = create_router(db, &config);
    assert!(format!("{:?}", router).contains("Router"));

    drop(temp_dir);
}

#[test]
fn test_create_router_with_openapi_enabled() {
    let (temp_dir, db) = create_test_db();
    let config = create_config(true, false);

    let router = create_router(db, &config);
    assert!(format!("{:?}", router).contains("Router"));

    drop(temp_dir);
}

#[test]
fn test_create_router_with_compression_enabled() {
    let (temp_dir, db) = create_test_db();
    let config = create_config(false, true);

    let router = create_router(db, &config);
    assert!(format!("{:?}", router).contains("Router"));

    drop(temp_dir);
}

#[test]
fn test_create_router_without_features() {
    let (temp_dir, db) = create_test_db();
    let config = create_config(false, false);

    let router = create_router(db, &config);
    assert!(format!("{:?}", router).contains("Router"));

    drop(temp_dir);
}
