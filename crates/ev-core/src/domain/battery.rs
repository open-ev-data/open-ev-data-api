use serde::{Deserialize, Serialize};

use super::enums::ThermalManagement;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Battery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub chemistry: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cathode_material: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pack_capacity_kwh_gross: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pack_capacity_kwh_net: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pack_voltage_nominal_v: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pack_voltage_max_v: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pack_voltage_min_v: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cell_count: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_count: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thermal_management: Option<ThermalManagement>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub heat_pump: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub preconditioning: Option<Preconditioning>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub warranty: Option<Warranty>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub usable_soc_window_percent: Option<UsableSocWindow>,
}

impl Battery {
    #[must_use]
    pub fn has_capacity(&self) -> bool {
        self.pack_capacity_kwh_gross.is_some() || self.pack_capacity_kwh_net.is_some()
    }

    #[must_use]
    pub fn usable_capacity_kwh(&self) -> Option<f64> {
        self.pack_capacity_kwh_net.or(self.pack_capacity_kwh_gross)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Preconditioning {
    pub supported: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub modes: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Warranty {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub years: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance_km: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_retention_percent: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UsableSocWindow {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_percent: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_percent: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}
