use serde::{Deserialize, Serialize};

use super::enums::{RangeCycle, RealWorldProfile};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Range {
    pub rated: Vec<RangeRated>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub real_world: Option<Vec<RangeRealWorld>>,
}

impl Range {
    #[must_use]
    pub fn wltp_range_km(&self) -> Option<f64> {
        self.rated
            .iter()
            .find(|r| r.cycle == RangeCycle::Wltp)
            .map(|r| r.range_km)
    }

    #[must_use]
    pub fn epa_range_km(&self) -> Option<f64> {
        self.rated
            .iter()
            .find(|r| r.cycle == RangeCycle::Epa)
            .map(|r| r.range_km)
    }

    #[must_use]
    pub fn best_rated_range_km(&self) -> Option<f64> {
        self.rated.iter().map(|r| r.range_km).reduce(f64::max)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RangeRated {
    pub cycle: RangeCycle,
    pub range_km: f64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RangeRealWorld {
    pub profile: RealWorldProfile,
    pub range_km: f64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<RealWorldConditions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RealWorldConditions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weather: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed_kmh: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Efficiency {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_consumption_wh_per_km: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpge: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl Efficiency {
    #[must_use]
    pub fn km_per_kwh(&self) -> Option<f64> {
        self.energy_consumption_wh_per_km.map(|wh| 1000.0 / wh)
    }
}
