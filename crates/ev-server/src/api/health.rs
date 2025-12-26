use std::sync::Arc;

use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};

use crate::db::Database;
use crate::error::{ApiError, ProblemDetails};
use crate::models::HealthResponse;

pub fn routes() -> Router<Arc<Database>> {
    Router::new().route("/health", get(health_check))
}

#[utoipa::path(
    get,
    path = "/health",
    tag = "health",
    summary = "Check API health status",
    description = "Returns the current health status of the API server, including database connectivity and vehicle count. Use this endpoint for monitoring and load balancer health checks.",
    responses(
        (status = 200, description = "Service is healthy and database is connected", body = HealthResponse),
        (status = 500, description = "Internal server error", body = ProblemDetails),
        (status = 503, description = "Service unavailable - database connection failed", body = ProblemDetails)
    )
)]
pub async fn health_check(
    State(db): State<Arc<Database>>,
) -> Result<Json<HealthResponse>, ApiError> {
    let vehicle_count = db
        .get_vehicle_count()
        .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

    Ok(Json(HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        database: "connected".to_string(),
        vehicle_count,
    }))
}
