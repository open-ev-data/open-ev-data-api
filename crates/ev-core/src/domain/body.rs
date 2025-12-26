use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Body {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub doors: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub seats: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rows: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub drag_coefficient_cd: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Dimensions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length_mm: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub width_mm: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub width_with_mirrors_mm: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub height_mm: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub wheelbase_mm: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ground_clearance_mm: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub turning_circle_m: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Weights {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub curb_weight_kg: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gross_vehicle_weight_kg: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_payload_kg: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub roof_load_kg: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Capacity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cargo_l: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cargo_max_l: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub frunk_l: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub towing_braked_kg: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub towing_unbraked_kg: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub towing_vertical_load_kg: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Performance {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acceleration_0_100_kmh_s: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub acceleration_0_60_mph_s: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_speed_kmh: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quarter_mile_s: Option<f64>,
}
