# ev-core

Core domain types and validation logic for OpenEV Data.

## Overview

This crate provides the foundational domain model for the OpenEV Data API, implementing a pure domain library with no I/O dependencies. It follows hexagonal architecture principles where this crate forms the inner hexagon.

## Features

- Type-safe domain entities matching the OpenEV Data JSON schema
- Comprehensive validation with detailed error messages
- Zero I/O dependencies (no HTTP, database, or file system access)
- Serde serialization/deserialization support
- Extensive unit tests

## Usage

```rust
use ev_core::{Vehicle, SlugName, Year, Validate};

// Create validated types
let make = SlugName::new("tesla", "Tesla")?;
let year = Year::new(2024)?;

// Validate a vehicle
let vehicle: Vehicle = serde_json::from_str(json_data)?;
vehicle.validate()?;
```

## Architecture

This crate is the Core of the Hexagonal Architecture:
- No dependencies on ev-etl or ev-server
- No I/O, HTTP, or database code
- All external interactions happen through trait boundaries in adapter crates
