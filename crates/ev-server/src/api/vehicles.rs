use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::routing::get;
use axum::{Json, Router};
use ev_core::Vehicle;

use crate::db::{Database, ListParams};
use crate::error::{ApiError, ProblemDetails};
use crate::models::{Pagination, VehicleListQuery, VehicleListResponse};

pub fn routes() -> Router<Arc<Database>> {
    Router::new()
        .route("/vehicles", get(list_vehicles))
        .route("/vehicles/code/{unique_code}", get(get_vehicle_by_code))
        .route("/vehicles/{make}/{model}/{year}", get(get_vehicle))
        .route(
            "/vehicles/{make}/{model}/{year}/variants",
            get(get_vehicle_variants),
        )
}

#[utoipa::path(
    get,
    path = "/vehicles",
    tag = "vehicles",
    summary = "List vehicles with filtering and pagination",
    description = "Browse all electric vehicles with optional filtering by manufacturer, model, year, type, and range. Results are paginated and can be sorted. Use this endpoint to discover available vehicles before fetching full details.",
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
    path = "/vehicles/{make}/{model}/{year}",
    tag = "vehicles",
    summary = "Get complete vehicle specifications",
    description = "Retrieve the full specification sheet for a specific vehicle, including battery, charging, performance, dimensions, and all technical details. This returns the base vehicle configuration; use the /variants endpoint for trim-specific data.",
    params(
        ("make" = String, Path, description = "Manufacturer slug (e.g., 'tesla')"),
        ("model" = String, Path, description = "Model slug (e.g., 'model_3')"),
        ("year" = u16, Path, description = "Model year (e.g., 2024)")
    ),
    responses(
        (status = 200, description = "Complete vehicle specification", body = inline(serde_json::Value), example = json!({
            "make": {"slug": "tesla", "name": "Tesla"},
            "model": {"slug": "model_3", "name": "Model 3"},
            "year": 2024,
            "trim_slug": "long_range",
            "trim_name": "Long Range",
            "vehicle_type": "bev",
            "battery": {"total_capacity_kwh": 82.0, "usable_capacity_kwh": 78.0},
            "range": {"wltp": [{"cycle": "combined", "range_km": 629}]},
            "charging": {"dc_max_power_kw": 250, "ac_max_power_kw": 11}
        })),
        (status = 404, description = "Vehicle not found", body = ProblemDetails),
        (status = 500, description = "Internal server error", body = ProblemDetails)
    )
)]
pub async fn get_vehicle(
    State(db): State<Arc<Database>>,
    Path((make, model, year)): Path<(String, String, u16)>,
) -> Result<Json<Vehicle>, ApiError> {
    let vehicle = db
        .get_vehicle(&make, &model, year)
        .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

    match vehicle {
        Some(v) => Ok(Json(v)),
        None => Err(ApiError::NotFound(format!(
            "Vehicle not found: {}/{}/{}",
            make, model, year
        ))),
    }
}

#[utoipa::path(
    get,
    path = "/vehicles/{make}/{model}/{year}/variants",
    tag = "vehicles",
    summary = "Get all variants/trims for a vehicle",
    description = "Retrieve all available trims and variants for a specific vehicle model year. Each variant may have different specifications (battery size, motor configuration, range, etc.). Returns an array of complete vehicle specifications.",
    params(
        ("make" = String, Path, description = "Manufacturer slug (e.g., 'tesla')"),
        ("model" = String, Path, description = "Model slug (e.g., 'model_3')"),
        ("year" = u16, Path, description = "Model year (e.g., 2024)")
    ),
    responses(
        (status = 200, description = "List of all vehicle variants", body = inline(Vec<serde_json::Value>), example = json!([{
            "make": {"slug": "tesla", "name": "Tesla"},
            "model": {"slug": "model_3", "name": "Model 3"},
            "year": 2024,
            "trim_slug": "long_range",
            "trim_name": "Long Range"
        }, {
            "make": {"slug": "tesla", "name": "Tesla"},
            "model": {"slug": "model_3", "name": "Model 3"},
            "year": 2024,
            "trim_slug": "performance",
            "trim_name": "Performance"
        }])),
        (status = 404, description = "No variants found for this vehicle", body = ProblemDetails),
        (status = 500, description = "Internal server error", body = ProblemDetails)
    )
)]
pub async fn get_vehicle_variants(
    State(db): State<Arc<Database>>,
    Path((make, model, year)): Path<(String, String, u16)>,
) -> Result<Json<Vec<Vehicle>>, ApiError> {
    let variants = db
        .get_vehicle_variants(&make, &model, year)
        .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

    if variants.is_empty() {
        return Err(ApiError::NotFound(format!(
            "No variants found for: {}/{}/{}",
            make, model, year
        )));
    }

    Ok(Json(variants))
}

#[utoipa::path(
    get,
    path = "/vehicles/code/{unique_code}",
    tag = "vehicles",
    summary = "Get vehicle by unique code",
    description = "Retrieve a vehicle using its unique identifier code. The code format is typically 'make-model-year-trim' (e.g., 'byd-dolphin-2024-standard'). This provides a direct lookup without needing separate make/model/year parameters.",
    params(
        ("unique_code" = String, Path, description = "Vehicle unique code (e.g., 'byd-dolphin-2024-standard')")
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
