use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::db::VehicleSummary;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[allow(dead_code)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub pagination: Pagination,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[schema(example = json!({
    "page": 1,
    "per_page": 20,
    "total": 150,
    "total_pages": 8
}))]
pub struct Pagination {
    pub page: usize,
    pub per_page: usize,
    pub total: usize,
    pub total_pages: usize,
}

impl Pagination {
    pub fn new(page: usize, per_page: usize, total: usize) -> Self {
        let total_pages = if total == 0 {
            0
        } else {
            (total + per_page - 1) / per_page
        };

        Self {
            page,
            per_page,
            total,
            total_pages,
        }
    }
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct VehicleListQuery {
    pub make: Option<String>,
    pub model: Option<String>,
    pub year: Option<u16>,
    pub vehicle_type: Option<String>,
    pub min_range_km: Option<f64>,
    pub max_range_km: Option<f64>,
    #[serde(default = "default_page")]
    pub page: usize,
    #[serde(default = "default_per_page")]
    pub per_page: usize,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

fn default_page() -> usize {
    1
}

fn default_per_page() -> usize {
    20
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[schema(example = json!({
    "vehicles": [{
        "id": 1,
        "unique_code": "tesla-model_3-2024-long_range",
        "make_slug": "tesla",
        "make_name": "Tesla",
        "model_slug": "model_3",
        "model_name": "Model 3",
        "year": 2024,
        "trim_name": "Long Range",
        "variant_name": null,
        "vehicle_type": "bev",
        "battery_capacity_kwh": 82.0,
        "range_wltp_km": 629.0,
        "range_epa_km": 533.0,
        "dc_max_power_kw": 250.0
    }],
    "pagination": {
        "page": 1,
        "per_page": 20,
        "total": 150,
        "total_pages": 8
    }
}))]
pub struct VehicleListResponse {
    pub vehicles: Vec<VehicleSummary>,
    pub pagination: Pagination,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[schema(example = json!({
    "makes": [{
        "slug": "tesla",
        "name": "Tesla",
        "vehicle_count": 25
    }, {
        "slug": "byd",
        "name": "BYD",
        "vehicle_count": 18
    }]
}))]
pub struct MakesListResponse {
    pub makes: Vec<crate::db::MakeSummary>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[schema(example = json!({
    "models": [{
        "slug": "model_3",
        "name": "Model 3",
        "years": [2024, 2023, 2022],
        "vehicle_count": 6
    }, {
        "slug": "model_y",
        "name": "Model Y",
        "years": [2024, 2023],
        "vehicle_count": 4
    }]
}))]
pub struct ModelsListResponse {
    pub models: Vec<crate::db::ModelSummary>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct SearchQuery {
    pub q: String,
    #[serde(default = "default_page")]
    pub page: usize,
    #[serde(default = "default_per_page")]
    pub per_page: usize,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[schema(example = json!({
    "results": [{
        "id": 5,
        "unique_code": "byd-dolphin-2024-standard",
        "make_slug": "byd",
        "make_name": "BYD",
        "model_slug": "dolphin",
        "model_name": "Dolphin",
        "year": 2024,
        "trim_name": "Standard",
        "variant_name": null,
        "vehicle_type": "bev",
        "battery_capacity_kwh": 44.9,
        "range_wltp_km": 340.0,
        "range_epa_km": null,
        "dc_max_power_kw": 60.0
    }],
    "pagination": {
        "page": 1,
        "per_page": 20,
        "total": 3,
        "total_pages": 1
    }
}))]
pub struct SearchResponse {
    pub results: Vec<VehicleSummary>,
    pub pagination: Pagination,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[schema(example = json!({
    "status": "healthy",
    "version": "0.1.0",
    "database": "connected",
    "vehicle_count": 150
}))]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub database: String,
    pub vehicle_count: usize,
}
