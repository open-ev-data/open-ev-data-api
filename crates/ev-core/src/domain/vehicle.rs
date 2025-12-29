use serde::{Deserialize, Serialize};

use super::battery::Battery;
use super::body::{Body, Capacity, Dimensions, Performance, Weights};
use super::charging::{ChargePort, Charging, V2X};
use super::enums::VehicleType;
use super::metadata::{Images, Links, Metadata, Pricing, Software, Variant, WheelsTires};
use super::powertrain::Powertrain;
use super::range::{Efficiency, Range};
use super::sources::Source;
use super::types::{SlugName, VehicleId, Year};
use crate::error::ValidationError;
use crate::validation::Validate;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Vehicle {
    #[serde(rename = "$schema", skip_serializing_if = "Option::is_none")]
    pub schema_url: Option<String>,
    pub schema_version: String,
    pub make: SlugName,
    pub model: SlugName,
    pub year: u16,
    pub trim: SlugName,
    pub vehicle_type: VehicleType,
    pub powertrain: Powertrain,
    pub battery: Battery,
    pub charge_ports: Vec<ChargePort>,
    pub charging: Charging,
    pub range: Range,
    pub sources: Vec<Source>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<Variant>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub markets: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability: Option<VehicleAvailability>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<Body>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Dimensions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub weights: Option<Weights>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<Capacity>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub v2x: Option<V2X>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub efficiency: Option<Efficiency>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance: Option<Performance>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub wheels_tires: Option<WheelsTires>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing: Option<Pricing>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub software: Option<Software>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Links>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Images>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VehicleAvailability {
    pub status: super::enums::AvailabilityStatus,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_year: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_year: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl Vehicle {
    #[must_use]
    pub fn id(&self) -> VehicleId {
        VehicleId {
            make_slug: self.make.slug.clone(),
            model_slug: self.model.slug.clone(),
            year: self.year,
            trim_slug: self.trim.slug.clone(),
            variant_slug: self.variant.as_ref().map(|v| v.slug.clone()),
        }
    }

    #[must_use]
    pub fn display_name(&self) -> String {
        format!(
            "{} {} {} {}",
            self.year, self.make.name, self.model.name, self.trim.name
        )
    }

    #[must_use]
    pub fn is_variant(&self) -> bool {
        self.variant.is_some()
    }

    #[must_use]
    pub fn usable_battery_kwh(&self) -> Option<f64> {
        self.battery.usable_capacity_kwh()
    }

    #[must_use]
    pub fn wltp_range_km(&self) -> Option<f64> {
        self.range.wltp_range_km()
    }

    #[must_use]
    pub fn epa_range_km(&self) -> Option<f64> {
        self.range.epa_range_km()
    }

    #[must_use]
    pub fn max_dc_power_kw(&self) -> Option<f64> {
        self.charging.dc.as_ref().map(|dc| dc.max_power_kw)
    }

    #[must_use]
    pub fn max_ac_power_kw(&self) -> Option<f64> {
        self.charging.ac.as_ref().map(|ac| ac.max_power_kw)
    }
}

impl Validate for Vehicle {
    fn validate(&self) -> Result<(), ValidationError> {
        let mut errors = Vec::new();

        if let Err(e) = self.make.validate() {
            errors.push(e);
        }

        if let Err(e) = self.model.validate() {
            errors.push(e);
        }

        if let Err(e) = Year::new(self.year) {
            errors.push(e);
        }

        if let Err(e) = self.trim.validate() {
            errors.push(e);
        }

        if !self.battery.has_capacity() {
            errors.push(ValidationError::MissingBatteryCapacity);
        }

        if self.charge_ports.is_empty() {
            errors.push(ValidationError::MissingChargePort);
        }

        if self.range.rated.is_empty() {
            errors.push(ValidationError::MissingRatedRange);
        }

        if self.sources.is_empty() {
            errors.push(ValidationError::MissingSource);
        }

        if errors.is_empty() {
            Ok(())
        } else if errors.len() == 1 {
            Err(errors.into_iter().next().expect("checked non-empty"))
        } else {
            Err(ValidationError::Multiple(errors))
        }
    }
}
