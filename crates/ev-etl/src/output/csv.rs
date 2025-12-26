use std::path::Path;

use anyhow::{Context, Result};
use ev_core::Vehicle;

pub fn generate(vehicles: &[Vehicle], output_path: &Path) -> Result<()> {
    let mut writer = csv::Writer::from_path(output_path)
        .with_context(|| format!("Failed to create CSV file at {:?}", output_path))?;

    writer.write_record([
        "unique_code",
        "make_slug",
        "make_name",
        "model_slug",
        "model_name",
        "year",
        "trim_slug",
        "trim_name",
        "variant_slug",
        "variant_name",
        "vehicle_type",
        "drivetrain",
        "system_power_kw",
        "system_torque_nm",
        "battery_capacity_gross_kwh",
        "battery_capacity_net_kwh",
        "battery_chemistry",
        "dc_max_power_kw",
        "ac_max_power_kw",
        "range_wltp_km",
        "range_epa_km",
        "acceleration_0_100_s",
        "top_speed_kmh",
        "charge_connectors",
        "sources",
    ])?;

    for vehicle in vehicles {
        let unique_code = vehicle
            .unique_code
            .clone()
            .unwrap_or_else(|| vehicle.id().canonical_id());

        let connectors: Vec<String> = vehicle
            .charge_ports
            .iter()
            .map(|p| format!("{:?}", p.connector))
            .collect();

        let sources: Vec<String> = vehicle.sources.iter().map(|s| s.url.clone()).collect();

        let acceleration = vehicle
            .performance
            .as_ref()
            .and_then(|p| p.acceleration_0_100_kmh_s);
        let top_speed = vehicle.performance.as_ref().and_then(|p| p.top_speed_kmh);

        writer.write_record([
            unique_code,
            vehicle.make.slug.clone(),
            vehicle.make.name.clone(),
            vehicle.model.slug.clone(),
            vehicle.model.name.clone(),
            vehicle.year.to_string(),
            vehicle.trim.slug.clone(),
            vehicle.trim.name.clone(),
            vehicle
                .variant
                .as_ref()
                .map(|v| v.slug.clone())
                .unwrap_or_default(),
            vehicle
                .variant
                .as_ref()
                .map(|v| v.name.clone())
                .unwrap_or_default(),
            format!("{:?}", vehicle.vehicle_type),
            format!("{:?}", vehicle.powertrain.drivetrain),
            vehicle
                .powertrain
                .system_power_kw
                .map(|v| v.to_string())
                .unwrap_or_default(),
            vehicle
                .powertrain
                .system_torque_nm
                .map(|v| v.to_string())
                .unwrap_or_default(),
            vehicle
                .battery
                .pack_capacity_kwh_gross
                .map(|v| v.to_string())
                .unwrap_or_default(),
            vehicle
                .battery
                .pack_capacity_kwh_net
                .map(|v| v.to_string())
                .unwrap_or_default(),
            vehicle.battery.chemistry.clone().unwrap_or_default(),
            vehicle
                .charging
                .dc
                .as_ref()
                .map(|dc| dc.max_power_kw.to_string())
                .unwrap_or_default(),
            vehicle
                .charging
                .ac
                .as_ref()
                .map(|ac| ac.max_power_kw.to_string())
                .unwrap_or_default(),
            vehicle
                .range
                .wltp_range_km()
                .map(|v| v.to_string())
                .unwrap_or_default(),
            vehicle
                .range
                .epa_range_km()
                .map(|v| v.to_string())
                .unwrap_or_default(),
            acceleration.map(|v| v.to_string()).unwrap_or_default(),
            top_speed.map(|v| v.to_string()).unwrap_or_default(),
            connectors.join("|"),
            sources.join("|"),
        ])?;
    }

    writer.flush()?;

    Ok(())
}
