use ev_server::db::{ListParams, SqliteDatabase};
use rusqlite::Connection;
use tempfile::NamedTempFile;

#[test]
fn test_sqlite_db_init() {
    let file = NamedTempFile::new().unwrap();
    let path = file.path().to_str().unwrap();
    let _db = SqliteDatabase::new(path).unwrap();
    // Verify connection is open - implicit by new succeeding
    // Also internal conn is private, so we test behavior via methods.
}

#[test]
fn test_get_vehicle_count() {
    let file = NamedTempFile::new().unwrap();
    let path = file.path().to_str().unwrap();

    // Init Schema
    {
        let conn = Connection::open(path).unwrap();
        conn.execute("CREATE TABLE vehicles (id INTEGER PRIMARY KEY)", [])
            .unwrap();
        conn.execute("INSERT INTO vehicles (id) VALUES (1), (2), (3)", [])
            .unwrap();
    }

    let db = SqliteDatabase::new(path).unwrap();
    assert_eq!(db.get_vehicle_count().unwrap(), 3);
}

#[test]
fn test_list_vehicles_filtering() {
    let file = NamedTempFile::new().unwrap();
    let path = file.path().to_str().unwrap();

    {
        let conn = Connection::open(path).unwrap();
        conn.execute(
            "CREATE TABLE vehicles (
                id INTEGER PRIMARY KEY,
                unique_code TEXT NOT NULL,
                make_slug TEXT NOT NULL,
                make_name TEXT NOT NULL,
                model_slug TEXT NOT NULL,
                model_name TEXT NOT NULL,
                year INTEGER NOT NULL,
                trim_name TEXT NOT NULL,
                variant_name TEXT,
                vehicle_type TEXT NOT NULL,
                battery_capacity_net_kwh REAL,
                range_wltp_km REAL,
                range_epa_km REAL,
                dc_max_power_kw REAL
            )",
            [],
        )
        .unwrap();

        conn.execute(
            "INSERT INTO vehicles (unique_code, make_slug, make_name, model_slug, model_name, year, trim_name, vehicle_type, range_wltp_km)
             VALUES ('t1', 'tesla', 'Tesla', 'model_3', 'Model 3', 2024, 'RWD', 'bev', 513.0)", []
        ).unwrap();
        conn.execute(
            "INSERT INTO vehicles (unique_code, make_slug, make_name, model_slug, model_name, year, trim_name, vehicle_type, range_wltp_km)
             VALUES ('t2', 'tesla', 'Tesla', 'model_y', 'Model Y', 2024, 'RWD', 'bev', 455.0)", []
        ).unwrap();
    }

    let db = SqliteDatabase::new(path).unwrap();

    // Test Make Filter
    let params = ListParams {
        make: Some("tesla".to_string()),
        ..Default::default()
    };
    let (vehicles, total) = db.list_vehicles(&params).unwrap();
    assert_eq!(total, 2);
    assert_eq!(vehicles.len(), 2);

    // Test Model Filter
    let params = ListParams {
        model: Some("model_3".to_string()),
        ..Default::default()
    };
    let (vehicles, total) = db.list_vehicles(&params).unwrap();
    assert_eq!(total, 1);
    assert_eq!(vehicles[0].model_slug, "model_3");

    // Test Range Filter
    let params = ListParams {
        min_range_km: Some(500.0),
        ..Default::default()
    };
    let (vehicles, total) = db.list_vehicles(&params).unwrap();
    assert_eq!(total, 1);
    assert_eq!(vehicles[0].unique_code, "t1");
}

#[test]
fn test_list_vehicles_year_filter() {
    let file = NamedTempFile::new().unwrap();
    let path = file.path().to_str().unwrap();

    {
        let conn = Connection::open(path).unwrap();
        conn.execute(
            "CREATE TABLE vehicles (
                id INTEGER PRIMARY KEY,
                unique_code TEXT NOT NULL,
                make_slug TEXT NOT NULL,
                make_name TEXT NOT NULL,
                model_slug TEXT NOT NULL,
                model_name TEXT NOT NULL,
                year INTEGER NOT NULL,
                trim_name TEXT NOT NULL,
                variant_name TEXT,
                vehicle_type TEXT NOT NULL,
                battery_capacity_net_kwh REAL,
                range_wltp_km REAL,
                range_epa_km REAL,
                dc_max_power_kw REAL
            )",
            [],
        )
        .unwrap();

        conn.execute(
            "INSERT INTO vehicles (unique_code, make_slug, make_name, model_slug, model_name, year, trim_name, vehicle_type)
             VALUES ('t1', 'tesla', 'Tesla', 'model_3', 'Model 3', 2024, 'RWD', 'bev')", []
        ).unwrap();
        conn.execute(
            "INSERT INTO vehicles (unique_code, make_slug, make_name, model_slug, model_name, year, trim_name, vehicle_type)
             VALUES ('t2', 'tesla', 'Tesla', 'model_3', 'Model 3', 2023, 'RWD', 'bev')", []
        ).unwrap();
    }

    let db = SqliteDatabase::new(path).unwrap();

    let params = ListParams {
        year: Some(2024),
        ..Default::default()
    };
    let (vehicles, total) = db.list_vehicles(&params).unwrap();
    assert_eq!(total, 1);
    assert_eq!(vehicles[0].year, 2024);
}

#[test]
fn test_list_vehicles_vehicle_type_filter() {
    let file = NamedTempFile::new().unwrap();
    let path = file.path().to_str().unwrap();

    {
        let conn = Connection::open(path).unwrap();
        conn.execute(
            "CREATE TABLE vehicles (
                id INTEGER PRIMARY KEY,
                unique_code TEXT NOT NULL,
                make_slug TEXT NOT NULL,
                make_name TEXT NOT NULL,
                model_slug TEXT NOT NULL,
                model_name TEXT NOT NULL,
                year INTEGER NOT NULL,
                trim_name TEXT NOT NULL,
                variant_name TEXT,
                vehicle_type TEXT NOT NULL,
                battery_capacity_net_kwh REAL,
                range_wltp_km REAL,
                range_epa_km REAL,
                dc_max_power_kw REAL
            )",
            [],
        )
        .unwrap();

        conn.execute(
            "INSERT INTO vehicles (unique_code, make_slug, make_name, model_slug, model_name, year, trim_name, vehicle_type)
             VALUES ('t1', 'tesla', 'Tesla', 'model_3', 'Model 3', 2024, 'RWD', 'bev')", []
        ).unwrap();
        conn.execute(
            "INSERT INTO vehicles (unique_code, make_slug, make_name, model_slug, model_name, year, trim_name, vehicle_type)
             VALUES ('t2', 'bmw', 'BMW', '330e', '330e', 2024, 'Base', 'phev')", []
        ).unwrap();
    }

    let db = SqliteDatabase::new(path).unwrap();

    let params = ListParams {
        vehicle_type: Some("bev".to_string()),
        ..Default::default()
    };
    let (vehicles, total) = db.list_vehicles(&params).unwrap();
    assert_eq!(total, 1);
    assert_eq!(vehicles[0].vehicle_type, "bev");
}

#[test]
fn test_list_vehicles_max_range_filter() {
    let file = NamedTempFile::new().unwrap();
    let path = file.path().to_str().unwrap();

    {
        let conn = Connection::open(path).unwrap();
        conn.execute(
            "CREATE TABLE vehicles (
                id INTEGER PRIMARY KEY,
                unique_code TEXT NOT NULL,
                make_slug TEXT NOT NULL,
                make_name TEXT NOT NULL,
                model_slug TEXT NOT NULL,
                model_name TEXT NOT NULL,
                year INTEGER NOT NULL,
                trim_name TEXT NOT NULL,
                variant_name TEXT,
                vehicle_type TEXT NOT NULL,
                battery_capacity_net_kwh REAL,
                range_wltp_km REAL,
                range_epa_km REAL,
                dc_max_power_kw REAL
            )",
            [],
        )
        .unwrap();

        conn.execute(
            "INSERT INTO vehicles (unique_code, make_slug, make_name, model_slug, model_name, year, trim_name, vehicle_type, range_wltp_km)
             VALUES ('t1', 'tesla', 'Tesla', 'model_3', 'Model 3', 2024, 'RWD', 'bev', 513.0)", []
        ).unwrap();
        conn.execute(
            "INSERT INTO vehicles (unique_code, make_slug, make_name, model_slug, model_name, year, trim_name, vehicle_type, range_wltp_km)
             VALUES ('t2', 'byd', 'BYD', 'dolphin', 'Dolphin', 2024, 'Base', 'bev', 340.0)", []
        ).unwrap();
    }

    let db = SqliteDatabase::new(path).unwrap();

    let params = ListParams {
        max_range_km: Some(400.0),
        ..Default::default()
    };
    let (vehicles, total) = db.list_vehicles(&params).unwrap();
    assert_eq!(total, 1);
    assert_eq!(vehicles[0].unique_code, "t2");
}

#[test]
fn test_list_vehicles_sorting() {
    let file = NamedTempFile::new().unwrap();
    let path = file.path().to_str().unwrap();

    {
        let conn = Connection::open(path).unwrap();
        conn.execute(
            "CREATE TABLE vehicles (
                id INTEGER PRIMARY KEY,
                unique_code TEXT NOT NULL,
                make_slug TEXT NOT NULL,
                make_name TEXT NOT NULL,
                model_slug TEXT NOT NULL,
                model_name TEXT NOT NULL,
                year INTEGER NOT NULL,
                trim_name TEXT NOT NULL,
                variant_name TEXT,
                vehicle_type TEXT NOT NULL,
                battery_capacity_net_kwh REAL,
                range_wltp_km REAL,
                range_epa_km REAL,
                dc_max_power_kw REAL
            )",
            [],
        )
        .unwrap();

        conn.execute(
            "INSERT INTO vehicles (unique_code, make_slug, make_name, model_slug, model_name, year, trim_name, vehicle_type, range_wltp_km)
             VALUES ('t1', 'tesla', 'Tesla', 'model_3', 'Model 3', 2024, 'RWD', 'bev', 513.0)", []
        ).unwrap();
        conn.execute(
            "INSERT INTO vehicles (unique_code, make_slug, make_name, model_slug, model_name, year, trim_name, vehicle_type, range_wltp_km)
             VALUES ('t2', 'byd', 'BYD', 'dolphin', 'Dolphin', 2024, 'Base', 'bev', 340.0)", []
        ).unwrap();
    }

    let db = SqliteDatabase::new(path).unwrap();

    // Sort by range descending
    let params = ListParams {
        sort_by: Some("range_wltp_km".to_string()),
        sort_order: Some("desc".to_string()),
        ..Default::default()
    };
    let (vehicles, _) = db.list_vehicles(&params).unwrap();
    assert_eq!(vehicles[0].unique_code, "t1"); // Higher range first

    // Sort by year ascending
    let params = ListParams {
        sort_by: Some("year".to_string()),
        sort_order: Some("asc".to_string()),
        ..Default::default()
    };
    let (vehicles, _) = db.list_vehicles(&params).unwrap();
    assert_eq!(vehicles.len(), 2);
}

#[test]
fn test_list_vehicles_pagination() {
    let file = NamedTempFile::new().unwrap();
    let path = file.path().to_str().unwrap();

    {
        let conn = Connection::open(path).unwrap();
        conn.execute(
            "CREATE TABLE vehicles (
                id INTEGER PRIMARY KEY,
                unique_code TEXT NOT NULL,
                make_slug TEXT NOT NULL,
                make_name TEXT NOT NULL,
                model_slug TEXT NOT NULL,
                model_name TEXT NOT NULL,
                year INTEGER NOT NULL,
                trim_name TEXT NOT NULL,
                variant_name TEXT,
                vehicle_type TEXT NOT NULL,
                battery_capacity_net_kwh REAL,
                range_wltp_km REAL,
                range_epa_km REAL,
                dc_max_power_kw REAL
            )",
            [],
        )
        .unwrap();

        for i in 1..=5 {
            conn.execute(
                &format!("INSERT INTO vehicles (unique_code, make_slug, make_name, model_slug, model_name, year, trim_name, vehicle_type) VALUES ('t{}', 'tesla', 'Tesla', 'model_3', 'Model 3', 2024, 'RWD', 'bev')", i), []
            ).unwrap();
        }
    }

    let db = SqliteDatabase::new(path).unwrap();

    // Page 1 with 2 per page
    let params = ListParams {
        page: 1,
        per_page: 2,
        ..Default::default()
    };
    let (vehicles, total) = db.list_vehicles(&params).unwrap();
    assert_eq!(total, 5);
    assert_eq!(vehicles.len(), 2);

    // Page 3 with 2 per page should have 1 result
    let params = ListParams {
        page: 3,
        per_page: 2,
        ..Default::default()
    };
    let (vehicles, total) = db.list_vehicles(&params).unwrap();
    assert_eq!(total, 5);
    assert_eq!(vehicles.len(), 1);
}
