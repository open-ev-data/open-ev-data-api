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
                .uri("/makes/list")
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
                .uri("/makes/list")
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

    // Check Tesla (should have models array)
    let tesla = makes.iter().find(|m| m["slug"] == "tesla").unwrap();
    assert_eq!(tesla["name"], "Tesla");
    assert_eq!(tesla["vehicle_count"], 2);
    // Check models array exists
    let models = tesla["models"].as_array().unwrap();
    assert!(models.contains(&serde_json::json!("Model 3")));
    assert!(models.contains(&serde_json::json!("Model Y")));

    // Check BMW
    let bmw = makes.iter().find(|m| m["slug"] == "bmw").unwrap();
    assert_eq!(bmw["name"], "BMW");
    assert_eq!(bmw["vehicle_count"], 1);
    let bmw_models = bmw["models"].as_array().unwrap();
    assert!(bmw_models.contains(&serde_json::json!("i4")));
}
