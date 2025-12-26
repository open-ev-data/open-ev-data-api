mod strategy;

use std::collections::HashMap;

use anyhow::Result;
use ev_core::Vehicle;
use serde_json::Value;

use crate::ingest::{FileType, VehicleFile};

pub use strategy::deep_merge;

pub fn merge_all(files: &[VehicleFile]) -> Result<Vec<Vehicle>> {
    let mut grouped: HashMap<(String, String), Vec<&VehicleFile>> = HashMap::new();

    for file in files {
        let key = (file.make_slug.clone(), file.model_slug.clone());
        grouped.entry(key).or_default().push(file);
    }

    let mut vehicles = Vec::new();

    for ((make_slug, model_slug), model_files) in grouped {
        let base_file = model_files.iter().find(|f| f.file_type == FileType::ModelBase);
        let base_content = base_file.map(|f| f.content.clone()).unwrap_or(Value::Object(serde_json::Map::new()));

        let mut years: HashMap<u16, Vec<&VehicleFile>> = HashMap::new();
        for file in model_files.iter().filter(|f| f.year.is_some()) {
            years.entry(file.year.expect("checked")).or_default().push(file);
        }

        for (year, year_files) in years {
            let year_base = year_files.iter().find(|f| f.file_type == FileType::YearBase);
            let variants: Vec<_> = year_files.iter().filter(|f| f.file_type == FileType::Variant).collect();

            if let Some(year_base_file) = year_base {
                let merged_base = deep_merge(&base_content, &year_base_file.content);

                match serde_json::from_value::<Vehicle>(merged_base.clone()) {
                    Ok(vehicle) => vehicles.push(vehicle),
                    Err(e) => {
                        tracing::warn!(
                            "Failed to parse base vehicle {}/{}/{}: {}",
                            make_slug, model_slug, year, e
                        );
                    }
                }

                for variant_file in variants {
                    let merged_variant = deep_merge(&merged_base, &variant_file.content);

                    match serde_json::from_value::<Vehicle>(merged_variant) {
                        Ok(vehicle) => vehicles.push(vehicle),
                        Err(e) => {
                            tracing::warn!(
                                "Failed to parse variant {:?}: {}",
                                variant_file.path, e
                            );
                        }
                    }
                }
            }
        }
    }

    vehicles.sort_by(|a, b| {
        a.make.slug.cmp(&b.make.slug)
            .then(a.model.slug.cmp(&b.model.slug))
            .then(a.year.cmp(&b.year))
            .then(a.trim.slug.cmp(&b.trim.slug))
    });

    Ok(vehicles)
}
