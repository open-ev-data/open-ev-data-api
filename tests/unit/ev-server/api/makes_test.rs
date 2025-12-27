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
async fn test_list_makes_empty() {
    let file = NamedTempFile::new().unwrap();
    let path = file.path().to_str().unwrap();

    // Init DB
    {
        let conn = Connection::open(path).unwrap();
        conn.execute(
            "CREATE TABLE vehicles (
                id INTEGER PRIMARY KEY,
                make_slug TEXT NOT NULL,
                make_name TEXT NOT NULL,
                model_slug TEXT NOT NULL,
                model_name TEXT NOT NULL
            )",
            [],
        )
        .unwrap();
    }

    let db = Arc::new(Database::new(path).unwrap());
    let app = ev_server::api::makes::routes().with_state(db);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/makes")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert!(json["makes"].as_array().unwrap().is_empty());
}

#[tokio::test]
async fn test_list_makes_populated() {
    let file = NamedTempFile::new().unwrap();
    let path = file.path().to_str().unwrap();

    {
        let conn = Connection::open(path).unwrap();
        conn.execute(
            "CREATE TABLE vehicles (
                id INTEGER PRIMARY KEY,
                make_slug TEXT NOT NULL,
                make_name TEXT NOT NULL,
                model_slug TEXT NOT NULL,
                model_name TEXT NOT NULL
            )",
            [],
        )
        .unwrap();

        conn.execute("INSERT INTO vehicles (make_slug, make_name, model_slug, model_name) VALUES ('tesla', 'Tesla', 'model_3', 'Model 3')", []).unwrap();
        conn.execute("INSERT INTO vehicles (make_slug, make_name, model_slug, model_name) VALUES ('tesla', 'Tesla', 'model_y', 'Model Y')", []).unwrap();
        conn.execute("INSERT INTO vehicles (make_slug, make_name, model_slug, model_name) VALUES ('bmw', 'BMW', 'i4', 'i4')", []).unwrap();
    }

    let db = Arc::new(Database::new(path).unwrap());
    let app = ev_server::api::makes::routes().with_state(db);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/makes")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    let makes = json["makes"].as_array().unwrap();
    assert_eq!(makes.len(), 2);

    // Check Tesla
    let tesla = makes.iter().find(|m| m["slug"] == "tesla").unwrap();
    assert_eq!(tesla["name"], "Tesla");
    assert_eq!(tesla["vehicle_count"], 2); // 2 models inserted (actually distinct vehicles)

    // Check BMW
    let bmw = makes.iter().find(|m| m["slug"] == "bmw").unwrap();
    assert_eq!(bmw["name"], "BMW");
    assert_eq!(bmw["vehicle_count"], 1);
}

#[tokio::test]
async fn test_list_models_for_make() {
    let file = NamedTempFile::new().unwrap();
    let path = file.path().to_str().unwrap();

    {
        let conn = Connection::open(path).unwrap();
        conn.execute(
            "CREATE TABLE vehicles (
                id INTEGER PRIMARY KEY,
                make_slug TEXT NOT NULL,
                make_name TEXT NOT NULL,
                model_slug TEXT NOT NULL,
                model_name TEXT NOT NULL,
                year INTEGER NOT NULL
            )",
            [],
        )
        .unwrap();

        conn.execute("INSERT INTO vehicles (make_slug, make_name, model_slug, model_name, year) VALUES ('tesla', 'Tesla', 'model_3', 'Model 3', 2024)", []).unwrap();
        conn.execute("INSERT INTO vehicles (make_slug, make_name, model_slug, model_name, year) VALUES ('tesla', 'Tesla', 'model_3', 'Model 3', 2023)", []).unwrap();
        conn.execute("INSERT INTO vehicles (make_slug, make_name, model_slug, model_name, year) VALUES ('tesla', 'Tesla', 'model_y', 'Model Y', 2024)", []).unwrap();
    }

    let db = Arc::new(Database::new(path).unwrap());
    let app = ev_server::api::makes::routes().with_state(db);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/makes/tesla/models")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    let models = json["models"].as_array().unwrap();
    assert_eq!(models.len(), 2);

    let model_3 = models.iter().find(|m| m["slug"] == "model_3").unwrap();
    assert_eq!(model_3["name"], "Model 3");
    // Check years are present
    let years = model_3["years"].as_array().unwrap();
    assert!(years.contains(&serde_json::json!(2024)));
    assert!(years.contains(&serde_json::json!(2023)));
}
