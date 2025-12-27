use axum::{http::StatusCode, response::IntoResponse};
use ev_server::error::{ApiError, ProblemDetails};
use http_body_util::BodyExt; // extension trait for Body

#[tokio::test]
async fn test_api_error_response_checking() {
    // 1. NotFound
    let err = ApiError::NotFound("item missing".into());
    let response = err.into_response();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let pd: ProblemDetails = serde_json::from_slice(&body_bytes).unwrap();
    assert_eq!(pd.status, 404);
    assert_eq!(pd.detail, "item missing");
    assert_eq!(pd.title, "Resource Not Found");

    // 2. BadRequest
    let err = ApiError::BadRequest("bad input".into());
    let response = err.into_response();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let pd: ProblemDetails = serde_json::from_slice(&body_bytes).unwrap();
    assert_eq!(pd.status, 400);
    assert!(pd.detail.contains("bad input"));
    assert_eq!(pd.title, "Invalid Request");

    // 3. InternalError
    let err = ApiError::InternalError("oops".into());
    let response = err.into_response();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let pd: ProblemDetails = serde_json::from_slice(&body_bytes).unwrap();
    assert_eq!(pd.status, 500);
    assert!(pd.detail.contains("oops"));

    // 4. DatabaseError
    let err = ApiError::DatabaseError("connection failed".into());
    let response = err.into_response();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let pd: ProblemDetails = serde_json::from_slice(&body_bytes).unwrap();
    assert_eq!(pd.status, 500);
    assert!(pd.detail.contains("Database error: connection failed"));

    // 5. ServiceUnavailable
    let err = ApiError::ServiceUnavailable("overloaded".into());
    let response = err.into_response();
    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let pd: ProblemDetails = serde_json::from_slice(&body_bytes).unwrap();
    assert_eq!(pd.status, 503);
    assert!(pd.detail.contains("overloaded"));
}

#[test]
fn test_problem_details_builder() {
    let pd = ProblemDetails::new("type", "title", StatusCode::BAD_REQUEST, "detail".into());
    assert_eq!(pd.instance, None);

    let pd_with_instance = pd.with_instance("/api/v1/resource");
    assert_eq!(pd_with_instance.instance, Some("/api/v1/resource".into()));
}

#[test]
fn test_error_conversions() {
    // any error from
    let anyhow_err = anyhow::anyhow!("some error");
    let api_err: ApiError = anyhow_err.into();
    if let ApiError::InternalError(msg) = api_err {
        assert_eq!(msg, "some error");
    } else {
        panic!("Expected InternalError");
    }

    // rusqlite error from
    let sqlite_err = rusqlite::Error::QueryReturnedNoRows;
    let api_err: ApiError = sqlite_err.into();
    if let ApiError::DatabaseError(msg) = api_err {
        assert_eq!(msg, rusqlite::Error::QueryReturnedNoRows.to_string());
    } else {
        panic!("Expected DatabaseError");
    }
}
