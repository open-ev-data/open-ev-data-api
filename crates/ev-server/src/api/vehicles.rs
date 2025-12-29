use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::routing::get;
use axum::{Json, Router};
use ev_core::Vehicle;

use crate::db::{Database, ListParams};
use crate::error::{ApiError, ProblemDetails};
use crate::models::{
    Pagination, SearchQuery, SearchResponse, VehicleListQuery, VehicleListResponse,
};

pub fn routes() -> Router<Arc<Database>> {
    Router::new()
        .route("/vehicles/list", get(list_vehicles))
        .route("/vehicles/code/{unique_code}", get(get_vehicle_by_code))
        .route("/vehicles/search", get(search_vehicles))
}

#[utoipa::path(
    get,
    path = "/vehicles/list",
    tag = "vehicles",
    summary = "List vehicles with filtering and pagination",
    description = "Browse all electric vehicles with optional filtering by manufacturer, model, year, type, and range. Results are paginated and can be sorted.",
    params(
        ("make" = Option<String>, Query, description = "Filter by manufacturer slug (e.g., 'tesla')"),
        ("model" = Option<String>, Query, description = "Filter by model slug (e.g., 'model_3')"),
        ("year" = Option<u16>, Query, description = "Filter by model year (e.g., 2024)"),
        ("vehicle_type" = Option<String>, Query, description = "Filter by vehicle type (e.g., 'bev', 'phev')"),
        ("min_range_km" = Option<f64>, Query, description = "Minimum WLTP range in kilometers"),
        ("max_range_km" = Option<f64>, Query, description = "Maximum WLTP range in kilometers"),
        ("page" = Option<usize>, Query, description = "Page number, starting from 1 (default: 1)"),
        ("per_page" = Option<usize>, Query, description = "Results per page, 1-100 (default: 20)"),
        ("sort_by" = Option<String>, Query, description = "Sort field: 'make', 'model', 'year', 'range'"),
        ("sort_order" = Option<String>, Query, description = "Sort direction: 'asc' or 'desc' (default: 'asc')")
    ),
    responses(
        (status = 200, description = "Paginated list of vehicle summaries", body = VehicleListResponse),
        (status = 400, description = "Invalid query parameters", body = ProblemDetails),
        (status = 500, description = "Internal server error", body = ProblemDetails)
    )
)]
pub async fn list_vehicles(
    State(db): State<Arc<Database>>,
    Query(query): Query<VehicleListQuery>,
) -> Result<Json<VehicleListResponse>, ApiError> {
    let per_page = query.per_page.min(100).max(1);
    let page = query.page.max(1);

    let params = ListParams {
        make: query.make,
        model: query.model,
        year: query.year,
        vehicle_type: query.vehicle_type,
        min_range_km: query.min_range_km,
        max_range_km: query.max_range_km,
        page,
        per_page,
        sort_by: query.sort_by,
        sort_order: query.sort_order,
    };

    let (vehicles, total) = db
        .list_vehicles(&params)
        .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

    Ok(Json(VehicleListResponse {
        vehicles,
        pagination: Pagination::new(page, per_page, total),
    }))
}

#[utoipa::path(
    get,
    path = "/vehicles/code/{unique_code}",
    tag = "vehicles",
    summary = "Get vehicle by unique code",
    description = "Retrieve a vehicle using its unique identifier code. The code format is 'make:model:year:filename' (e.g., 'byd:dolphin:2024:dolphin').",
    params(
        ("unique_code" = String, Path, description = "Vehicle unique code (e.g., 'byd:dolphin:2024:dolphin')")
    ),
    responses(
        (status = 200, description = "Vehicle found", body = inline(serde_json::Value), example = json!({
            "make": {"slug": "byd", "name": "BYD"},
            "model": {"slug": "dolphin", "name": "Dolphin"},
            "year": 2024,
            "trim_slug": "standard",
            "trim_name": "Standard",
            "vehicle_type": "bev"
        })),
        (status = 404, description = "Vehicle not found", body = ProblemDetails),
        (status = 500, description = "Internal server error", body = ProblemDetails)
    )
)]
pub async fn get_vehicle_by_code(
    State(db): State<Arc<Database>>,
    Path(unique_code): Path<String>,
) -> Result<Json<Vehicle>, ApiError> {
    let vehicle = db
        .get_vehicle_by_code(&unique_code)
        .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

    match vehicle {
        Some(v) => Ok(Json(v)),
        None => Err(ApiError::NotFound(format!(
            "Vehicle not found with code: {}",
            unique_code
        ))),
    }
}

#[utoipa::path(
    get,
    path = "/vehicles/search",
    tag = "vehicles",
    summary = "Search vehicles by keyword",
    description = "Full-text search across vehicle makes, models, and trims. Returns paginated results matching the query. The search is case-insensitive and matches partial words. Minimum query length is 2 characters.",
    params(
        ("q" = String, Query, description = "Search query (minimum 2 characters)"),
        ("page" = Option<usize>, Query, description = "Page number, starting from 1 (default: 1)"),
        ("per_page" = Option<usize>, Query, description = "Results per page, 1-100 (default: 20)")
    ),
    responses(
        (status = 200, description = "Search results with pagination", body = SearchResponse),
        (status = 400, description = "Invalid search query (empty or too short)", body = ProblemDetails),
        (status = 500, description = "Internal server error", body = ProblemDetails)
    )
)]
pub async fn search_vehicles(
    State(db): State<Arc<Database>>,
    Query(query): Query<SearchQuery>,
) -> Result<Json<SearchResponse>, ApiError> {
    if query.q.is_empty() {
        return Err(ApiError::BadRequest(
            "Search query cannot be empty".to_string(),
        ));
    }

    if query.q.len() < 2 {
        return Err(ApiError::BadRequest(
            "Search query must be at least 2 characters".to_string(),
        ));
    }

    let per_page = query.per_page.min(100).max(1);
    let page = query.page.max(1);

    let (vehicles, total) = db
        .search(&query.q, page, per_page)
        .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

    Ok(Json(SearchResponse {
        results: vehicles,
        pagination: Pagination::new(page, per_page, total),
    }))
}
