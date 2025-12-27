use axum::{body::Body, http::Request};
use ev_server::db::Database;
use http_body_util::BodyExt;
use rusqlite::Connection;
use std::sync::Arc;
use tempfile::NamedTempFile;
use tower::ServiceExt;

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
    let app = ev_server::api::search::routes().with_state(db);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/search?q=tesla")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["results"].as_array().unwrap().len(), 0);
}

#[tokio::test]
async fn test_search_vehicles_match() {
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
            "INSERT INTO vehicles (unique_code, make_slug, make_name, model_slug, model_name, year, trim_name, vehicle_type) VALUES ('tesla-model_3-2024-rwd', 'tesla', 'Tesla', 'model_3', 'Model 3', 2024, 'RWD', 'bev')",
            [],
        )
        .unwrap();

        conn.execute(
            "INSERT INTO vehicles (unique_code, make_slug, make_name, model_slug, model_name, year, trim_name, vehicle_type) VALUES ('bmw-i4-2024-edrive40', 'bmw', 'BMW', 'i4', 'i4', 2024, 'eDrive40', 'bev')",
            [],
        )
        .unwrap();
    }

    let db = Arc::new(Database::new(path).unwrap());
    let app = ev_server::api::search::routes().with_state(db);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/search?q=model")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    let results = json["results"].as_array().unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0]["model_name"], "Model 3");
}
