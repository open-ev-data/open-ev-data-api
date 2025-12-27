use std::path::Path;

use anyhow::{Context, Result};
use ev_core::Vehicle;
use rusqlite::{Connection, params};

pub fn generate(vehicles: &[Vehicle], output_path: &Path) -> Result<()> {
    if output_path.exists() {
        std::fs::remove_file(output_path)?;
    }

    let conn = Connection::open(output_path)
        .with_context(|| format!("Failed to create SQLite database at {:?}", output_path))?;

    create_schema(&conn)?;
    insert_vehicles(&conn, vehicles)?;
    create_indexes(&conn)?;
    optimize_database(&conn)?;

    Ok(())
}

fn create_schema(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r"
        CREATE TABLE IF NOT EXISTS vehicles (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            unique_code TEXT UNIQUE,
            make_slug TEXT NOT NULL,
            make_name TEXT NOT NULL,
            model_slug TEXT NOT NULL,
            model_name TEXT NOT NULL,
            year INTEGER NOT NULL,
            trim_slug TEXT NOT NULL,
            trim_name TEXT NOT NULL,
            variant_slug TEXT,
            variant_name TEXT,
            vehicle_type TEXT NOT NULL,
            drivetrain TEXT NOT NULL,
            system_power_kw REAL,
            system_torque_nm REAL,
            battery_capacity_gross_kwh REAL,
            battery_capacity_net_kwh REAL,
            battery_chemistry TEXT,
            dc_max_power_kw REAL,
            ac_max_power_kw REAL,
            range_wltp_km REAL,
            range_epa_km REAL,
            acceleration_0_100_s REAL,
            top_speed_kmh REAL,
            json_data TEXT NOT NULL,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS charge_ports (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            vehicle_id INTEGER NOT NULL,
            kind TEXT NOT NULL,
            connector TEXT NOT NULL,
            location_side TEXT,
            location_position TEXT,
            FOREIGN KEY (vehicle_id) REFERENCES vehicles(id)
        );

        CREATE TABLE IF NOT EXISTS range_ratings (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            vehicle_id INTEGER NOT NULL,
            cycle TEXT NOT NULL,
            range_km REAL NOT NULL,
            notes TEXT,
            FOREIGN KEY (vehicle_id) REFERENCES vehicles(id)
        );

        CREATE TABLE IF NOT EXISTS sources (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            vehicle_id INTEGER NOT NULL,
            source_type TEXT NOT NULL,
            title TEXT NOT NULL,
            url TEXT NOT NULL,
            accessed_at TEXT NOT NULL,
            publisher TEXT,
            FOREIGN KEY (vehicle_id) REFERENCES vehicles(id)
        );
        ",
    )
    .context("Failed to create database schema")?;

    Ok(())
}

fn insert_vehicles(conn: &Connection, vehicles: &[Vehicle]) -> Result<()> {
    let mut stmt = conn.prepare(
        r"INSERT INTO vehicles (
            unique_code, make_slug, make_name, model_slug, model_name,
            year, trim_slug, trim_name, variant_slug, variant_name,
            vehicle_type, drivetrain, system_power_kw, system_torque_nm,
            battery_capacity_gross_kwh, battery_capacity_net_kwh, battery_chemistry,
            dc_max_power_kw, ac_max_power_kw, range_wltp_km, range_epa_km,
            acceleration_0_100_s, top_speed_kmh, json_data
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24)",
    )?;

    let mut port_stmt = conn.prepare(
        "INSERT INTO charge_ports (vehicle_id, kind, connector, location_side, location_position) VALUES (?1, ?2, ?3, ?4, ?5)",
    )?;

    let mut range_stmt = conn.prepare(
        "INSERT INTO range_ratings (vehicle_id, cycle, range_km, notes) VALUES (?1, ?2, ?3, ?4)",
    )?;

    let mut source_stmt = conn.prepare(
        "INSERT INTO sources (vehicle_id, source_type, title, url, accessed_at, publisher) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
    )?;

    for vehicle in vehicles {
        let json_data = serde_json::to_string(vehicle)?;
        let unique_code = vehicle
            .unique_code
            .clone()
            .unwrap_or_else(|| vehicle.id().canonical_id());

        let acceleration = vehicle
            .performance
            .as_ref()
            .and_then(|p| p.acceleration_0_100_kmh_s);
        let top_speed = vehicle.performance.as_ref().and_then(|p| p.top_speed_kmh);

        stmt.execute(params![
            unique_code,
            vehicle.make.slug,
            vehicle.make.name,
            vehicle.model.slug,
            vehicle.model.name,
            vehicle.year,
            vehicle.trim.slug,
            vehicle.trim.name,
            vehicle.variant.as_ref().map(|v| &v.slug),
            vehicle.variant.as_ref().map(|v| &v.name),
            format!("{:?}", vehicle.vehicle_type),
            format!("{:?}", vehicle.powertrain.drivetrain),
            vehicle.powertrain.system_power_kw,
            vehicle.powertrain.system_torque_nm,
            vehicle.battery.pack_capacity_kwh_gross,
            vehicle.battery.pack_capacity_kwh_net,
            vehicle.battery.chemistry,
            vehicle.charging.dc.as_ref().map(|dc| dc.max_power_kw),
            vehicle.charging.ac.as_ref().map(|ac| ac.max_power_kw),
            vehicle.range.wltp_range_km(),
            vehicle.range.epa_range_km(),
            acceleration,
            top_speed,
            json_data,
        ])?;

        let vehicle_id = conn.last_insert_rowid();

        for port in &vehicle.charge_ports {
            let location_side = port
                .location
                .as_ref()
                .and_then(|l| l.side.as_ref())
                .map(|s| format!("{:?}", s));
            let location_pos = port
                .location
                .as_ref()
                .and_then(|l| l.position.as_ref())
                .map(|p| format!("{:?}", p));

            port_stmt.execute(params![
                vehicle_id,
                format!("{:?}", port.kind),
                format!("{:?}", port.connector),
                location_side,
                location_pos,
            ])?;
        }

        for rating in &vehicle.range.rated {
            range_stmt.execute(params![
                vehicle_id,
                format!("{:?}", rating.cycle),
                rating.range_km,
                rating.notes,
            ])?;
        }

        for source in &vehicle.sources {
            source_stmt.execute(params![
                vehicle_id,
                format!("{:?}", source.source_type),
                source.title,
                source.url,
                source.accessed_at,
                source.publisher,
            ])?;
        }
    }

    Ok(())
}

fn create_indexes(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r"
        CREATE INDEX IF NOT EXISTS idx_vehicles_make ON vehicles(make_slug);
        CREATE INDEX IF NOT EXISTS idx_vehicles_model ON vehicles(model_slug);
        CREATE INDEX IF NOT EXISTS idx_vehicles_year ON vehicles(year);
        CREATE INDEX IF NOT EXISTS idx_vehicles_composite ON vehicles(make_slug, model_slug, year, trim_slug);
        CREATE INDEX IF NOT EXISTS idx_vehicles_type ON vehicles(vehicle_type);
        CREATE INDEX IF NOT EXISTS idx_charge_ports_vehicle ON charge_ports(vehicle_id);
        CREATE INDEX IF NOT EXISTS idx_range_ratings_vehicle ON range_ratings(vehicle_id);
        CREATE INDEX IF NOT EXISTS idx_sources_vehicle ON sources(vehicle_id);
        ",
    )
    .context("Failed to create indexes")?;

    Ok(())
}

fn optimize_database(conn: &Connection) -> Result<()> {
    conn.execute_batch("VACUUM; ANALYZE;")
        .context("Failed to optimize database")?;

    Ok(())
}
