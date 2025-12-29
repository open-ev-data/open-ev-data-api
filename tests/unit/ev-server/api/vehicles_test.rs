use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use ev_server::db::Database;
use http_body_util::BodyExt;
use rusqlite::Connection;
use std::sync::Arc;
use tempfile::NamedTempFile;
use tower::ServiceExt;

#[tokio::test]
async fn test_list_vehicles_pagination() {
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

        for i in 1..=25 {
            conn.execute(
                "INSERT INTO vehicles (unique_code, make_slug, make_name, model_slug, model_name, year, trim_name, vehicle_type)
                 VALUES (?1, 'make', 'Make', 'model', 'Model', 2024, 'Trim', 'bev')",
                [format!("code-{}", i)],
            )
            .unwrap();
        }
    }

    let db = Arc::new(Database::new(path).unwrap());
    let app = ev_server::api::vehicles::routes().with_state(db);

    // Test default pagination (page 1, per_page 20)
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/vehicles/list")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["vehicles"].as_array().unwrap().len(), 20);
    assert_eq!(json["pagination"]["total_pages"], 2);
    assert_eq!(json["pagination"]["total"], 25);

    // Test page 2
    let response = app
        .oneshot(
            Request::builder()
                .uri("/vehicles/list?page=2&per_page=20")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["vehicles"].as_array().unwrap().len(), 5);
}

#[tokio::test]
async fn test_get_vehicle_by_code_found() {
    let file = NamedTempFile::new().unwrap();
    let path = file.path().to_str().unwrap();

    {
        let conn = Connection::open(path).unwrap();
        conn.execute(
            "CREATE TABLE vehicles (
                id INTEGER PRIMARY KEY,
                unique_code TEXT NOT NULL,
                json_data TEXT NOT NULL
            )",
            [],
        )
        .unwrap();

        let vehicle_json = serde_json::json!({
             "schema_version": "1.0",
             "make": {"slug": "byd", "name": "BYD"},
             "model": {"slug": "dolphin", "name": "Dolphin"},
             "year": 2024,
             "trim": {"slug": "standard", "name": "Standard"},
             "vehicle_type": "passenger_car",
             "powertrain": { "drivetrain": "fwd" },
             "battery": {},
             "charging": {},
             "range": { "rated": [] },
             "sources": [],
             "charge_ports": []
        })
        .to_string();

        conn.execute(
            "INSERT INTO vehicles (unique_code, json_data) VALUES ('byd:dolphin:2024:dolphin', ?)",
            [vehicle_json],
        )
        .unwrap();
    }

    let db = Arc::new(Database::new(path).unwrap());
    let app = ev_server::api::vehicles::routes().with_state(db);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/vehicles/code/byd:dolphin:2024:dolphin")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["model"]["name"], "Dolphin");
}

#[tokio::test]
async fn test_get_vehicle_by_code_not_found() {
    let file = NamedTempFile::new().unwrap();
    let path = file.path().to_str().unwrap();

    {
        let conn = Connection::open(path).unwrap();
        conn.execute(
            "CREATE TABLE vehicles (
                id INTEGER PRIMARY KEY,
                unique_code TEXT NOT NULL,
                json_data TEXT NOT NULL
            )",
            [],
        )
        .unwrap();
    }

    let db = Arc::new(Database::new(path).unwrap());
    let app = ev_server::api::vehicles::routes().with_state(db);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/vehicles/code/missing-code")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_search_vehicles_empty() {
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
    }

    let db = Arc::new(Database::new(path).unwrap());
    let app = ev_server::api::vehicles::routes().with_state(db);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/vehicles/search?q=tesla")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert!(json["results"].as_array().unwrap().is_empty());
}

#[tokio::test]
async fn test_search_vehicles_bad_request() {
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
    }

    let db = Arc::new(Database::new(path).unwrap());
    let app = ev_server::api::vehicles::routes().with_state(db);

    // Test empty query returns bad request
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/vehicles/search?q=")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    // Test short query returns bad request
    let response = app
        .oneshot(
            Request::builder()
                .uri("/vehicles/search?q=a")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
