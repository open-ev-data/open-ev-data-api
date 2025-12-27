mod battery;
mod body;
mod charging;
pub mod enums;
mod metadata;
mod powertrain;
mod range;
mod sources;
mod types;
mod vehicle;

pub use battery::{Battery, Preconditioning, UsableSocWindow, Warranty};
pub use body::{Body, Capacity, Dimensions, Weights};
pub use charging::{
    ChargeCurve, ChargeCurvePoint, ChargePort, Charging, ChargingAc, ChargingDc, ChargingProtocols,
    ChargingTime, Conditions,
};
pub use metadata::{Images, Links, Metadata, Variant};
pub use powertrain::{Motor, Powertrain, Transmission};
pub use range::{Efficiency, Range, RangeRated, RangeRealWorld};
pub use sources::Source;
pub use types::{SlugName, VehicleId, Year};
pub use vehicle::{Vehicle, VehicleAvailability};

pub use charging::{V2LOutlet, V2G, V2H, V2L, V2X};
pub use metadata::{Msrp, Pricing, Software, WheelsTires};
