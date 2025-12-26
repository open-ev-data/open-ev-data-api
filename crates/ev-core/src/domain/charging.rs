use serde::{Deserialize, Serialize};

use super::enums::{
    ChargeCurveType, ChargerVoltageClass, ConnectorType, PortKind, PortLocation, PortPosition,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChargePort {
    pub kind: PortKind,
    pub connector: ConnectorType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<ChargePortLocation>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub covers: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub light: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub motorized: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChargePortLocation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<PortLocation>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<PortPosition>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Charging {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ac: Option<ChargingAc>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dc: Option<ChargingDc>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocols: Option<ChargingProtocols>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dc_charge_curve: Option<ChargeCurve>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_time: Option<ChargingTime>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChargingAc {
    pub max_power_kw: f64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_power_steps_kw: Option<Vec<f64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub phases: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub voltage_range_v: Option<VoltageRange>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_hz: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_current_a: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub onboard_charger_count: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChargingDc {
    pub max_power_kw: f64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub voltage_range_v: Option<VoltageRange>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_current_a: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture_voltage_class: Option<ChargerVoltageClass>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_limits_by_voltage: Option<Vec<PowerLimitByVoltage>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VoltageRange {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_v: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_v: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PowerLimitByVoltage {
    pub voltage_class: String,
    pub max_power_kw: f64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ChargingProtocols {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ac: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dc: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub plug_and_charge: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChargeCurve {
    pub curve_type: ChargeCurveType,
    pub points: Vec<ChargeCurvePoint>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Conditions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChargeCurvePoint {
    pub soc_percent: f64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_kw: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_a: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub voltage_v: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Conditions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub battery_temp_c: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ambient_temp_c: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub preconditioning: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub charger_power_kw: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ChargingTime {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ac: Option<Vec<ChargingTimeEntry>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dc: Option<Vec<DcChargingTimeEntry>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChargingTimeEntry {
    pub power_kw: f64,
    pub from_soc_percent: f64,
    pub to_soc_percent: f64,
    pub time_min: f64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Conditions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DcChargingTimeEntry {
    pub charger_power_kw: f64,
    pub from_soc_percent: f64,
    pub to_soc_percent: f64,
    pub time_min: f64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Conditions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct V2X {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v2l: Option<V2L>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub v2h: Option<V2H>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub v2g: Option<V2G>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct V2L {
    pub supported: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_power_kw: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub outlets: Option<Vec<V2LOutlet>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct V2LOutlet {
    pub kind: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct V2H {
    pub supported: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_power_kw: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocols: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct V2G {
    pub supported: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_power_kw: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocols: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}
