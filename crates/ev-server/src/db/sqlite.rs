use std::sync::Mutex;

use anyhow::{Context, Result};
use ev_core::Vehicle;
use rusqlite::{Connection, params};

use super::{ListParams, MakeSummary, VehicleSummary};

fn build_order_clause(sort_by: &Option<String>, sort_order: &Option<String>) -> String {
    let valid_columns = [
        "year",
        "make_name",
        "model_name",
        "range_wltp_km",
        "range_epa_km",
        "battery_capacity_net_kwh",
        "dc_max_power_kw",
    ];
    let column = sort_by
        .as_ref()
        .filter(|s| valid_columns.contains(&s.as_str()))
        .map(|s| s.as_str())
        .unwrap_or("make_slug, model_slug, year");

    let direction = sort_order
        .as_ref()
        .filter(|s| s.as_str() == "desc" || s.as_str() == "asc")
        .map(|s| s.as_str())
        .unwrap_or("asc");

    format!("ORDER BY {} {}", column, direction.to_uppercase())
}

pub struct SqliteDatabase {
    conn: Mutex<Connection>,
}

impl SqliteDatabase {
    pub fn new(path: &str) -> Result<Self> {
        let conn =
            Connection::open(path).with_context(|| format!("Failed to open database: {}", path))?;

        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    pub fn get_vehicle_count(&self) -> Result<usize> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
        let count: i64 = conn.query_row("SELECT COUNT(*) FROM vehicles", [], |row| row.get(0))?;
        Ok(count as usize)
    }

    pub fn list_vehicles(&self, params: &ListParams) -> Result<(Vec<VehicleSummary>, usize)> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;

        let mut conditions = Vec::new();
        let mut sql_params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        if let Some(ref make) = params.make {
            conditions.push("make_slug = ?");
            sql_params.push(Box::new(make.clone()));
        }

        if let Some(ref model) = params.model {
            conditions.push("model_slug = ?");
            sql_params.push(Box::new(model.clone()));
        }

        if let Some(year) = params.year {
            conditions.push("year = ?");
            sql_params.push(Box::new(year as i32));
        }

        if let Some(ref vehicle_type) = params.vehicle_type {
            conditions.push("vehicle_type = ?");
            sql_params.push(Box::new(vehicle_type.clone()));
        }

        if let Some(min_range) = params.min_range_km {
            conditions.push("(range_wltp_km >= ? OR range_epa_km >= ?)");
            sql_params.push(Box::new(min_range));
            sql_params.push(Box::new(min_range));
        }

        if let Some(max_range) = params.max_range_km {
            conditions.push("(range_wltp_km <= ? OR range_epa_km <= ?)");
            sql_params.push(Box::new(max_range));
            sql_params.push(Box::new(max_range));
        }

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        let count_sql = format!("SELECT COUNT(*) FROM vehicles {}", where_clause);
        let params_refs: Vec<&dyn rusqlite::ToSql> =
            sql_params.iter().map(|p| p.as_ref()).collect();
        let total: i64 = conn.query_row(&count_sql, params_refs.as_slice(), |row| row.get(0))?;

        let offset = (params.page.saturating_sub(1)) * params.per_page;

        let order_clause = build_order_clause(&params.sort_by, &params.sort_order);
        let query_sql = format!(
            "SELECT id, unique_code, make_slug, make_name, model_slug, model_name, year, trim_name, variant_name, vehicle_type, battery_capacity_net_kwh, range_wltp_km, range_epa_km, dc_max_power_kw FROM vehicles {} {} LIMIT ? OFFSET ?",
            where_clause, order_clause
        );

        let mut query_params = sql_params;
        query_params.push(Box::new(params.per_page as i32));
        query_params.push(Box::new(offset as i32));

        let params_refs: Vec<&dyn rusqlite::ToSql> =
            query_params.iter().map(|p| p.as_ref()).collect();
        let mut stmt = conn.prepare(&query_sql)?;

        let vehicles: Vec<VehicleSummary> = stmt
            .query_map(params_refs.as_slice(), |row| {
                Ok(VehicleSummary {
                    id: row.get(0)?,
                    unique_code: row.get(1)?,
                    make_slug: row.get(2)?,
                    make_name: row.get(3)?,
                    model_slug: row.get(4)?,
                    model_name: row.get(5)?,
                    year: row.get::<_, i32>(6)? as u16,
                    trim_name: row.get(7)?,
                    variant_name: row.get(8)?,
                    vehicle_type: row.get(9)?,
                    battery_capacity_kwh: row.get(10)?,
                    range_wltp_km: row.get(11)?,
                    range_epa_km: row.get(12)?,
                    dc_max_power_kw: row.get(13)?,
                })
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok((vehicles, total as usize))
    }

    pub fn list_makes(&self) -> Result<Vec<MakeSummary>> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;

        let mut stmt = conn.prepare(
            "SELECT make_slug, make_name, COUNT(*) as count FROM vehicles GROUP BY make_slug, make_name ORDER BY make_name",
        )?;

        let mut makes: Vec<MakeSummary> = stmt
            .query_map([], |row| {
                Ok(MakeSummary {
                    slug: row.get(0)?,
                    name: row.get(1)?,
                    vehicle_count: row.get::<_, i64>(2)? as usize,
                    models: vec![],
                })
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;

        for make in &mut makes {
            let mut model_stmt = conn.prepare(
                "SELECT DISTINCT model_name FROM vehicles WHERE make_slug = ? ORDER BY model_name",
            )?;
            make.models = model_stmt
                .query_map(params![make.slug], |row| row.get(0))?
                .filter_map(|r| r.ok())
                .collect();
        }

        Ok(makes)
    }

    pub fn search(
        &self,
        query: &str,
        page: usize,
        per_page: usize,
    ) -> Result<(Vec<VehicleSummary>, usize)> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;

        let search_pattern = format!("%{}%", query.to_lowercase());

        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM vehicles WHERE LOWER(make_name) LIKE ? OR LOWER(model_name) LIKE ? OR LOWER(trim_name) LIKE ?",
            params![&search_pattern, &search_pattern, &search_pattern],
            |row| row.get(0),
        )?;

        let offset = (page.saturating_sub(1)) * per_page;

        let mut stmt = conn.prepare(
            "SELECT id, unique_code, make_slug, make_name, model_slug, model_name, year, trim_name, variant_name, vehicle_type, battery_capacity_net_kwh, range_wltp_km, range_epa_km, dc_max_power_kw FROM vehicles WHERE LOWER(make_name) LIKE ? OR LOWER(model_name) LIKE ? OR LOWER(trim_name) LIKE ? ORDER BY make_slug, model_slug, year LIMIT ? OFFSET ?",
        )?;

        let vehicles: Vec<VehicleSummary> = stmt
            .query_map(
                params![
                    &search_pattern,
                    &search_pattern,
                    &search_pattern,
                    per_page as i32,
                    offset as i32
                ],
                |row| {
                    Ok(VehicleSummary {
                        id: row.get(0)?,
                        unique_code: row.get(1)?,
                        make_slug: row.get(2)?,
                        make_name: row.get(3)?,
                        model_slug: row.get(4)?,
                        model_name: row.get(5)?,
                        year: row.get::<_, i32>(6)? as u16,
                        trim_name: row.get(7)?,
                        variant_name: row.get(8)?,
                        vehicle_type: row.get(9)?,
                        battery_capacity_kwh: row.get(10)?,
                        range_wltp_km: row.get(11)?,
                        range_epa_km: row.get(12)?,
                        dc_max_power_kw: row.get(13)?,
                    })
                },
            )?
            .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok((vehicles, count as usize))
    }

    pub fn get_vehicle_by_code(&self, code: &str) -> Result<Option<Vehicle>> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;

        let result: Option<String> = conn
            .query_row(
                "SELECT json_data FROM vehicles WHERE unique_code = ? LIMIT 1",
                params![code],
                |row| row.get(0),
            )
            .optional()?;

        match result {
            Some(json) => {
                let vehicle: Vehicle = serde_json::from_str(&json)?;
                Ok(Some(vehicle))
            }
            None => Ok(None),
        }
    }
}

trait OptionalExt<T> {
    fn optional(self) -> Result<Option<T>, rusqlite::Error>;
}

impl<T> OptionalExt<T> for std::result::Result<T, rusqlite::Error> {
    fn optional(self) -> Result<Option<T>, rusqlite::Error> {
        match self {
            Ok(val) => Ok(Some(val)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        }
    }
}
