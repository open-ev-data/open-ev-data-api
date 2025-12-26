use serde::{Deserialize, Serialize};

use super::enums::{Drivetrain, MotorPosition};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Powertrain {
    pub drivetrain: Drivetrain,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_power_kw: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_torque_nm: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub motors: Option<Vec<Motor>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transmission: Option<Transmission>,
}

impl Powertrain {
    #[must_use]
    pub fn motor_count(&self) -> usize {
        self.motors.as_ref().map_or(0, Vec::len)
    }

    #[must_use]
    pub fn total_power_kw(&self) -> Option<f64> {
        self.system_power_kw.or_else(|| {
            self.motors.as_ref().map(|motors| {
                motors
                    .iter()
                    .filter_map(|m| m.power_kw)
                    .sum()
            })
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Motor {
    pub position: MotorPosition,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub motor_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_kw: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub torque_nm: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cooling: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transmission {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gears: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub transmission_type: Option<String>,
}
