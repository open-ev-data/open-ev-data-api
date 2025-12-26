use std::collections::HashMap;
use std::time::Duration;

use ev_core::Vehicle;
use serde::Serialize;

#[derive(Serialize)]
pub struct Statistics {
    pub total_vehicles: usize,
    pub makes: usize,
    pub models: usize,
    pub year_range: YearRange,
    pub vehicles_by_type: HashMap<String, usize>,
    pub vehicles_by_make: HashMap<String, usize>,
    pub processing_time_seconds: f64,
    pub etl_version: String,
}

#[derive(Serialize)]
pub struct YearRange {
    pub min: u16,
    pub max: u16,
}

pub fn generate(vehicles: &[Vehicle], processing_time: Duration) -> Statistics {
    let mut makes = std::collections::HashSet::new();
    let mut models = std::collections::HashSet::new();
    let mut vehicles_by_type: HashMap<String, usize> = HashMap::new();
    let mut vehicles_by_make: HashMap<String, usize> = HashMap::new();
    let mut min_year = u16::MAX;
    let mut max_year = u16::MIN;

    for vehicle in vehicles {
        makes.insert(vehicle.make.slug.clone());
        models.insert((vehicle.make.slug.clone(), vehicle.model.slug.clone()));

        let vehicle_type = format!("{:?}", vehicle.vehicle_type);
        *vehicles_by_type.entry(vehicle_type).or_insert(0) += 1;

        *vehicles_by_make
            .entry(vehicle.make.name.clone())
            .or_insert(0) += 1;

        min_year = min_year.min(vehicle.year);
        max_year = max_year.max(vehicle.year);
    }

    Statistics {
        total_vehicles: vehicles.len(),
        makes: makes.len(),
        models: models.len(),
        year_range: YearRange {
            min: if min_year == u16::MAX { 0 } else { min_year },
            max: if max_year == u16::MIN { 0 } else { max_year },
        },
        vehicles_by_type,
        vehicles_by_make,
        processing_time_seconds: processing_time.as_secs_f64(),
        etl_version: env!("CARGO_PKG_VERSION").to_string(),
    }
}
