mod postgresql;
mod sqlite;

use anyhow::Result;
use ev_core::Vehicle;

pub use sqlite::SqliteDatabase;

pub struct Database {
    inner: SqliteDatabase,
}

impl Database {
    pub fn new(url: &str) -> Result<Self> {
        let inner = SqliteDatabase::new(url)?;
        Ok(Self { inner })
    }

    pub fn get_vehicle_count(&self) -> Result<usize> {
        self.inner.get_vehicle_count()
    }

    pub fn list_vehicles(&self, params: &ListParams) -> Result<(Vec<VehicleSummary>, usize)> {
        self.inner.list_vehicles(params)
    }

    pub fn get_vehicle(&self, make: &str, model: &str, year: u16) -> Result<Option<Vehicle>> {
        self.inner.get_vehicle(make, model, year)
    }

    pub fn get_vehicle_variants(&self, make: &str, model: &str, year: u16) -> Result<Vec<Vehicle>> {
        self.inner.get_vehicle_variants(make, model, year)
    }

    pub fn list_makes(&self) -> Result<Vec<MakeSummary>> {
        self.inner.list_makes()
    }

    pub fn list_models(&self, make: &str) -> Result<Vec<ModelSummary>> {
        self.inner.list_models(make)
    }

    pub fn search(
        &self,
        query: &str,
        page: usize,
        per_page: usize,
    ) -> Result<(Vec<VehicleSummary>, usize)> {
        self.inner.search(query, page, per_page)
    }

    pub fn get_vehicle_by_code(&self, code: &str) -> Result<Option<Vehicle>> {
        self.inner.get_vehicle_by_code(code)
    }
}

#[derive(Debug, Clone)]
pub struct ListParams {
    pub make: Option<String>,
    pub model: Option<String>,
    pub year: Option<u16>,
    pub vehicle_type: Option<String>,
    pub min_range_km: Option<f64>,
    pub max_range_km: Option<f64>,
    pub page: usize,
    pub per_page: usize,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

impl Default for ListParams {
    fn default() -> Self {
        Self {
            make: None,
            model: None,
            year: None,
            vehicle_type: None,
            min_range_km: None,
            max_range_km: None,
            page: 1,
            per_page: 20,
            sort_by: None,
            sort_order: None,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, utoipa::ToSchema)]
#[schema(example = json!({
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
}))]
pub struct VehicleSummary {
    pub id: i64,
    pub unique_code: String,
    pub make_slug: String,
    pub make_name: String,
    pub model_slug: String,
    pub model_name: String,
    pub year: u16,
    pub trim_name: String,
    pub variant_name: Option<String>,
    pub vehicle_type: String,
    pub battery_capacity_kwh: Option<f64>,
    pub range_wltp_km: Option<f64>,
    pub range_epa_km: Option<f64>,
    pub dc_max_power_kw: Option<f64>,
}

#[derive(Debug, Clone, serde::Serialize, utoipa::ToSchema)]
#[schema(example = json!({
    "slug": "tesla",
    "name": "Tesla",
    "vehicle_count": 25
}))]
pub struct MakeSummary {
    pub slug: String,
    pub name: String,
    pub vehicle_count: usize,
}

#[derive(Debug, Clone, serde::Serialize, utoipa::ToSchema)]
#[schema(example = json!({
    "slug": "model_3",
    "name": "Model 3",
    "years": [2024, 2023, 2022],
    "vehicle_count": 6
}))]
pub struct ModelSummary {
    pub slug: String,
    pub name: String,
    pub years: Vec<u16>,
    pub vehicle_count: usize,
}
