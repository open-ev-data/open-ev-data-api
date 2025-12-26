use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug)]
pub enum ApiError {
    NotFound(String),
    BadRequest(String),
    InternalError(String),
    DatabaseError(String),
    #[allow(dead_code)]
    ServiceUnavailable(String),
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[schema(example = json!({
    "type": "https://github.com/open-ev-data/open-ev-data-api/blob/main/docs/API_ERRORS.md#errorsnot-found",
    "title": "Resource Not Found",
    "status": 404,
    "detail": "Vehicle not found: tesla/model_x/2099",
    "instance": "/api/v1/vehicles/tesla/model_x/2099"
}))]
pub struct ProblemDetails {
    #[serde(rename = "type")]
    pub error_type: String,
    pub title: String,
    pub status: u16,
    pub detail: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
}

impl ProblemDetails {
    pub fn new(error_type: &str, title: &str, status: StatusCode, detail: String) -> Self {
        Self {
            error_type: error_type.to_string(),
            title: title.to_string(),
            status: status.as_u16(),
            detail,
            instance: None,
        }
    }

    #[allow(dead_code)]
    pub fn with_instance(mut self, instance: &str) -> Self {
        self.instance = Some(instance.to_string());
        self
    }
}

impl ApiError {
    fn to_problem_details(&self) -> (StatusCode, ProblemDetails) {
        match self {
            Self::NotFound(detail) => (
                StatusCode::NOT_FOUND,
                ProblemDetails::new(
                    "https://github.com/open-ev-data/open-ev-data-api/blob/main/docs/API_ERRORS.md#errorsnot-found",
                    "Resource Not Found",
                    StatusCode::NOT_FOUND,
                    detail.clone(),
                ),
            ),
            Self::BadRequest(detail) => (
                StatusCode::BAD_REQUEST,
                ProblemDetails::new(
                    "https://github.com/open-ev-data/open-ev-data-api/blob/main/docs/API_ERRORS.md#errorsbad-request",
                    "Invalid Request",
                    StatusCode::BAD_REQUEST,
                    detail.clone(),
                ),
            ),
            Self::InternalError(detail) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ProblemDetails::new(
                    "https://github.com/open-ev-data/open-ev-data-api/blob/main/docs/API_ERRORS.md#errorsinternal-error",
                    "Internal Server Error",
                    StatusCode::INTERNAL_SERVER_ERROR,
                    detail.clone(),
                ),
            ),
            Self::DatabaseError(detail) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ProblemDetails::new(
                    "https://github.com/open-ev-data/open-ev-data-api/blob/main/docs/API_ERRORS.md#errorsinternal-error",
                    "Internal Server Error",
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Database error: {}", detail),
                ),
            ),
            Self::ServiceUnavailable(detail) => (
                StatusCode::SERVICE_UNAVAILABLE,
                ProblemDetails::new(
                    "https://github.com/open-ev-data/open-ev-data-api/blob/main/docs/API_ERRORS.md#errorsservice-unavailable",
                    "Service Unavailable",
                    StatusCode::SERVICE_UNAVAILABLE,
                    detail.clone(),
                ),
            ),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, problem) = self.to_problem_details();
        (status, Json(problem)).into_response()
    }
}

impl From<rusqlite::Error> for ApiError {
    fn from(err: rusqlite::Error) -> Self {
        Self::DatabaseError(err.to_string())
    }
}

impl From<anyhow::Error> for ApiError {
    fn from(err: anyhow::Error) -> Self {
        Self::InternalError(err.to_string())
    }
}
