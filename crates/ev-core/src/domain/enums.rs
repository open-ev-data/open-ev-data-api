use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VehicleType {
    PassengerCar,
    Suv,
    Pickup,
    Van,
    Bus,
    Motorcycle,
    Scooter,
    Commercial,
    Truck,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Drivetrain {
    Fwd,
    Rwd,
    Awd,
    #[serde(rename = "4wd")]
    FourWd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MotorPosition {
    Front,
    Rear,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ThermalManagement {
    Liquid,
    Air,
    Passive,
    Refrigerant,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PortKind {
    AcOnly,
    DcOnly,
    Combo,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConnectorType {
    Type1,
    Type2,
    Ccs1,
    Ccs2,
    Nacs,
    Chademo,
    GbTAc,
    GbTDc,
    TeslaType2,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PortLocation {
    Left,
    Right,
    Front,
    Rear,
    Center,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PortPosition {
    Front,
    Rear,
    Mid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChargerVoltageClass {
    #[serde(rename = "400v")]
    V400,
    #[serde(rename = "800v")]
    V800,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChargeCurveType {
    PowerBySoc,
    CurrentBySoc,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RangeCycle {
    Wltp,
    Epa,
    Nedc,
    Cltc,
    Jc08,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RealWorldProfile {
    Highway,
    City,
    Mixed,
    ColdWeather,
    Winter,
    Summer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AvailabilityStatus {
    Production,
    Discontinued,
    Concept,
    Announced,
    Prototype,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceType {
    Oem,
    Regulatory,
    Press,
    Community,
    TestingOrg,
}

impl std::fmt::Display for VehicleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PassengerCar => write!(f, "Passenger Car"),
            Self::Suv => write!(f, "SUV"),
            Self::Pickup => write!(f, "Pickup"),
            Self::Van => write!(f, "Van"),
            Self::Bus => write!(f, "Bus"),
            Self::Motorcycle => write!(f, "Motorcycle"),
            Self::Scooter => write!(f, "Scooter"),
            Self::Commercial => write!(f, "Commercial"),
            Self::Truck => write!(f, "Truck"),
            Self::Other => write!(f, "Other"),
        }
    }
}

impl std::fmt::Display for Drivetrain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fwd => write!(f, "FWD"),
            Self::Rwd => write!(f, "RWD"),
            Self::Awd => write!(f, "AWD"),
            Self::FourWd => write!(f, "4WD"),
        }
    }
}

impl std::fmt::Display for ConnectorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Type1 => write!(f, "Type 1"),
            Self::Type2 => write!(f, "Type 2"),
            Self::Ccs1 => write!(f, "CCS1"),
            Self::Ccs2 => write!(f, "CCS2"),
            Self::Nacs => write!(f, "NACS"),
            Self::Chademo => write!(f, "CHAdeMO"),
            Self::GbTAc => write!(f, "GB/T AC"),
            Self::GbTDc => write!(f, "GB/T DC"),
            Self::TeslaType2 => write!(f, "Tesla Type 2"),
            Self::Other => write!(f, "Other"),
        }
    }
}

impl std::fmt::Display for RangeCycle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Wltp => write!(f, "WLTP"),
            Self::Epa => write!(f, "EPA"),
            Self::Nedc => write!(f, "NEDC"),
            Self::Cltc => write!(f, "CLTC"),
            Self::Jc08 => write!(f, "JC08"),
            Self::Other => write!(f, "Other"),
        }
    }
}
