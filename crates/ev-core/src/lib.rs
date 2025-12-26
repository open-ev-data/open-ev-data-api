//! ev-core: Domain types and validation for OpenEV Data.
//!
//! This crate provides the foundational domain model for the OpenEV Data API,
//! implementing a pure domain library with no I/O dependencies.
#![forbid(unsafe_code)]

pub mod domain;
pub mod error;
pub mod validation;

pub use domain::{
    Battery, Body, Capacity, ChargeCurve, ChargeCurvePoint, ChargePort, Charging, ChargingAc,
    ChargingDc, ChargingProtocols, ChargingTime, Conditions, Dimensions, Efficiency, Images, Links,
    Metadata, Motor, Msrp, Powertrain, Preconditioning, Pricing, Range, RangeRated, RangeRealWorld,
    SlugName, Source, Transmission, UsableSocWindow, V2LOutlet, Variant, Vehicle,
    VehicleAvailability, VehicleId, Warranty, Weights, WheelsTires, Year, V2G, V2H, V2L, V2X,
};

pub use domain::enums::{
    AvailabilityStatus, ChargeCurveType, ChargerVoltageClass, ConnectorType, Drivetrain,
    MotorPosition, PortKind, PortLocation, PortPosition, RangeCycle, RealWorldProfile, SourceType,
    ThermalManagement, VehicleType,
};

pub use error::{CoreError, ValidationError};
pub use validation::Validate;
