use serde::{Deserialize, Serialize};

use super::enums::AvailabilityStatus;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Variant {
    pub slug: String,
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct VehicleAvailability {
    pub status: AvailabilityStatus,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_year: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_year: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Pricing {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msrp: Option<Vec<Msrp>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Msrp {
    pub currency: String,
    pub amount: f64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Software {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ota_supported: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Links {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub press_kit_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec_sheet_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub configurator_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Images {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exterior_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub interior_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_curve_plot_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Metadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub contributors: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_quality: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_notes: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct WheelsTires {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard_wheel_size_in: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional_wheel_sizes_in: Option<Vec<f64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tire_sizes: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended_pressure_kpa: Option<TirePressure>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TirePressure {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front_kpa: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rear_kpa: Option<f64>,
}
