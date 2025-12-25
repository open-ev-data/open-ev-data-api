# Rust Development Guidelines - OpenEV Data API

This document establishes the coding standards, best practices, and architectural guidelines for developing the OpenEV Data API in Rust. All contributors must follow these guidelines to ensure code quality, maintainability, and consistency.

## Table of Contents

- [1. Core Principles](#1-core-principles)
- [2. Project Structure](#2-project-structure)
- [3. Naming Conventions](#3-naming-conventions)
- [4. Type Design](#4-type-design)
- [5. Error Handling](#5-error-handling)
- [6. Architectural Patterns](#6-architectural-patterns)
- [7. Async/Await Patterns](#7-asyncawait-patterns)
- [8. Testing Strategy](#8-testing-strategy)
- [9. Performance Guidelines](#9-performance-guidelines)
- [10. Security Practices](#10-security-practices)
- [11. Documentation Standards](#11-documentation-standards)
- [12. Code Quality Tools](#12-code-quality-tools)
- [13. Common Patterns](#13-common-patterns)
- [14. Anti-Patterns to Avoid](#14-anti-patterns-to-avoid)

---

## 1. Core Principles

### 1.1. SOLID Principles in Rust

#### Single Responsibility Principle (SRP)
Each module, struct, and function should have ONE clear responsibility.

**Good:**
```rust
// Each struct has a single responsibility
pub struct VehicleReader {
    // Only reads vehicle files
}

pub struct VehicleValidator {
    // Only validates vehicle data
}

pub struct VehicleRepository {
    // Only handles vehicle persistence
}
```

**Bad:**
```rust
// This struct does too many things
pub struct VehicleManager {
    // Reads, validates, transforms, saves, queries...
}
```

#### Open/Closed Principle (OCP)
Types should be open for extension but closed for modification. Use traits for extensibility.

**Good:**
```rust
// Define behavior through traits
pub trait OutputGenerator {
    fn generate(&self, vehicles: &[Vehicle]) -> Result<Vec<u8>>;
}

// Easy to add new formats without modifying existing code
pub struct JsonGenerator;
impl OutputGenerator for JsonGenerator { /* ... */ }

pub struct SqliteGenerator;
impl OutputGenerator for SqliteGenerator { /* ... */ }
```

#### Liskov Substitution Principle (LSP)
Implementations should be substitutable for their trait bounds without breaking behavior.

**Good:**
```rust
pub trait VehicleStore {
    fn save(&mut self, vehicle: Vehicle) -> Result<()>;
    fn find(&self, id: &str) -> Result<Option<Vehicle>>;
}

// Both implementations respect the contract
impl VehicleStore for SqliteStore { /* ... */ }
impl VehicleStore for PostgresStore { /* ... */ }
```

#### Interface Segregation Principle (ISP)
Clients should not depend on traits they don't use. Keep traits focused.

**Good:**
```rust
// Segregated traits
pub trait Readable {
    fn read(&self, id: &str) -> Result<Vehicle>;
}

pub trait Writable {
    fn write(&mut self, vehicle: Vehicle) -> Result<()>;
}

// Implement only what you need
impl Readable for ReadOnlyStore { /* ... */ }
impl Readable + Writable for FullStore { /* ... */ }
```

**Bad:**
```rust
// Fat trait forcing unnecessary implementations
pub trait Repository {
    fn read(&self, id: &str) -> Result<Vehicle>;
    fn write(&mut self, vehicle: Vehicle) -> Result<()>;
    fn delete(&mut self, id: &str) -> Result<()>;
    fn update(&mut self, vehicle: Vehicle) -> Result<()>;
}
```

#### Dependency Inversion Principle (DIP)
Depend on abstractions (traits), not concrete types.

**Good:**
```rust
// Depend on trait, not concrete type
pub struct EtlPipeline<G: OutputGenerator> {
    generator: G,
}

impl<G: OutputGenerator> EtlPipeline<G> {
    pub fn new(generator: G) -> Self {
        Self { generator }
    }
}
```

**Bad:**
```rust
// Hardcoded dependency on concrete type
pub struct EtlPipeline {
    generator: JsonGenerator, // Tightly coupled
}
```

### 1.2. Hexagonal Architecture Enforcement

The codebase is divided into layers with strict dependency rules:

```
┌─────────────────────────────────────┐
│         Adapters (Outer)            │
│  (ev-etl, ev-server)               │
│         ↓ depends on               │
├─────────────────────────────────────┤
│         Core (Inner)                │
│         (ev-core)                   │
│    ← no dependencies               │
└─────────────────────────────────────┘
```

**Rules:**
1. **ev-core** must NOT depend on ev-etl or ev-server
2. **ev-core** must NOT import any I/O, HTTP, or database crates
3. **ev-etl** and **ev-server** can depend on ev-core
4. **Adapters communicate with Core through trait boundaries**

### 1.3. Make Invalid States Unrepresentable

Use Rust's type system to prevent invalid states at compile time.

**Good:**
```rust
// Impossible to have a vehicle without required fields
pub struct Vehicle {
    pub make: SlugName,          // Cannot be empty
    pub model: SlugName,
    pub year: Year,              // Validated range (1900-2100)
    pub battery: BatterySpecs,   // At least one capacity required
}

// Year is constrained
pub struct Year(u16);

impl Year {
    pub fn new(value: u16) -> Result<Self, ValidationError> {
        if (1900..=2100).contains(&value) {
            Ok(Year(value))
        } else {
            Err(ValidationError::InvalidYear(value))
        }
    }
}
```

**Bad:**
```rust
// Many ways to be invalid
pub struct Vehicle {
    pub make: Option<String>,      // Could be None
    pub model: Option<String>,
    pub year: i32,                 // Could be negative or 9999
    pub battery: Option<Battery>,  // Could be None
}
```

---

## 2. Project Structure

### 2.1. Workspace Organization

```
Cargo.toml                    # Workspace manifest
├── crates/
│   ├── ev-core/             # Domain logic (no dependencies)
│   │   ├── src/
│   │   │   ├── lib.rs       # Public API surface
│   │   │   ├── domain/      # Domain entities
│   │   │   ├── validation/  # Validation logic
│   │   │   └── error.rs     # Core error types
│   │   └── Cargo.toml
│   │
│   ├── ev-etl/              # CLI adapter
│   │   ├── src/
│   │   │   ├── main.rs      # Binary entry
│   │   │   ├── cli.rs       # Argument parsing
│   │   │   ├── ingest/      # Input adapters
│   │   │   ├── output/      # Output adapters
│   │   │   └── error.rs     # ETL-specific errors
│   │   └── Cargo.toml
│   │
│   └── ev-server/           # HTTP API adapter
│       ├── src/
│       │   ├── main.rs      # Binary entry
│       │   ├── api/         # HTTP handlers
│       │   ├── db/          # Database adapters
│       │   └── error.rs     # Server-specific errors
│       └── Cargo.toml
```

### 2.2. Module Organization

Each crate should follow this internal structure:

```rust
// lib.rs or main.rs - Define public API
pub mod domain;
pub mod validation;

pub use domain::{Vehicle, Battery};  // Re-export public types
pub use validation::Validator;

// Private modules
mod internal;
```

### 2.3. File Naming

- **snake_case** for file names: `vehicle_repository.rs`
- **mod.rs** for module entry points
- One public type per file (generally)
- Related types can share a file if small

---

## 3. Naming Conventions

### 3.1. General Rules

| Item | Convention | Example |
|------|-----------|---------|
| Crates | kebab-case | `ev-core`, `ev-etl` |
| Modules | snake_case | `vehicle_store`, `json_parser` |
| Types | PascalCase | `Vehicle`, `BatterySpecs` |
| Traits | PascalCase | `OutputGenerator`, `Validator` |
| Functions | snake_case | `load_vehicles()`, `validate_schema()` |
| Methods | snake_case | `.to_json()`, `.save()` |
| Constants | SCREAMING_SNAKE_CASE | `MAX_VEHICLES`, `DEFAULT_PORT` |
| Statics | SCREAMING_SNAKE_CASE | `GLOBAL_CONFIG` |
| Lifetimes | lowercase, short | `'a`, `'b`, `'static` |
| Type parameters | PascalCase, single letter | `T`, `E`, or descriptive: `TOutput` |

### 3.2. Semantic Naming

**Types should indicate their purpose:**
```rust
// Good - Clear intent
pub struct VehicleRepository;
pub struct JsonSerializer;
pub struct ValidationError;

// Bad - Vague or generic
pub struct Manager;
pub struct Handler;
pub struct Data;
```

**Functions should be verbs or verb phrases:**
```rust
// Good
fn load_dataset() -> Result<Dataset>;
fn validate_vehicle(v: &Vehicle) -> Result<()>;
fn merge_json_files(files: &[File]) -> Json;

// Bad
fn dataset() -> Result<Dataset>;
fn vehicle(v: &Vehicle) -> Result<()>;
```

**Avoid abbreviations unless widely known:**
```rust
// Good
http_client, database_url, repository

// Acceptable
url, json, xml, html, api

// Bad
veh, db_repo, usr
```

---

## 4. Type Design

### 4.1. Prefer Newtype Pattern for Domain Concepts

Wrap primitive types to add semantic meaning and validation.

```rust
// Good - Type-safe and self-documenting
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VehicleId(String);

impl VehicleId {
    pub fn new(value: String) -> Result<Self, ValidationError> {
        if value.is_empty() {
            return Err(ValidationError::EmptyId);
        }
        Ok(VehicleId(value))
    }
}

// Bad - Easy to confuse different strings
fn get_vehicle(id: String) -> Vehicle;
fn get_make(make: String) -> Vec<Vehicle>;
// Which string is which?
```

### 4.2. Use Builder Pattern for Complex Construction

```rust
#[derive(Debug)]
pub struct VehicleBuilder {
    make: Option<SlugName>,
    model: Option<SlugName>,
    year: Option<Year>,
    battery: Option<BatterySpecs>,
}

impl VehicleBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn make(mut self, make: SlugName) -> Self {
        self.make = Some(make);
        self
    }

    pub fn build(self) -> Result<Vehicle, BuilderError> {
        Ok(Vehicle {
            make: self.make.ok_or(BuilderError::MissingMake)?,
            model: self.model.ok_or(BuilderError::MissingModel)?,
            year: self.year.ok_or(BuilderError::MissingYear)?,
            battery: self.battery.ok_or(BuilderError::MissingBattery)?,
        })
    }
}
```

### 4.3. Enums for State Representation

Use enums to represent mutually exclusive states.

```rust
// Good - Clear states
#[derive(Debug, Clone, PartialEq)]
pub enum VehicleAvailability {
    Production { start_year: u16 },
    Discontinued { end_year: u16 },
    Concept,
    Announced { expected_year: u16 },
}

// Bad - Multiple booleans
pub struct VehicleAvailability {
    pub is_production: bool,
    pub is_discontinued: bool,
    pub is_concept: bool,
    // Unclear which combination is valid
}
```

### 4.4. Derive Standard Traits

Always derive standard traits when possible:

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]          // For most types
#[derive(Debug, Clone, PartialEq)]                    // For types with floats
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]   // For small copyable types
```

For public domain types, also derive:
```rust
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
```

---

## 5. Error Handling

### 5.1. Error Design

Use `thiserror` for library errors (ev-core) and `anyhow` for application errors (ev-etl, ev-server).

**ev-core (library errors):**
```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("Missing required field: {field}")]
    MissingField { field: String },

    #[error("Invalid year: {0}. Must be between 1900 and 2100")]
    InvalidYear(u16),

    #[error("At least one battery capacity (gross or net) is required")]
    MissingBatteryCapacity,

    #[error("Schema validation failed: {0}")]
    SchemaError(String),
}
```

**ev-etl, ev-server (application errors):**
```rust
use anyhow::{Context, Result};

pub fn load_dataset(path: &Path) -> Result<Dataset> {
    let file = File::open(path)
        .context(format!("Failed to open dataset file: {:?}", path))?;
    
    let dataset = serde_json::from_reader(file)
        .context("Failed to parse JSON")?;
    
    Ok(dataset)
}
```

### 5.2. Result Type Usage

Always use `Result` for operations that can fail:

```rust
// Good
pub fn validate_vehicle(vehicle: &Vehicle) -> Result<(), ValidationError>;
pub fn save_to_db(&self, vehicle: Vehicle) -> Result<VehicleId>;

// Bad - panicking in library code
pub fn validate_vehicle(vehicle: &Vehicle) {
    assert!(!vehicle.make.is_empty());  // NO!
}

// Bad - hiding errors
pub fn save_to_db(&self, vehicle: Vehicle) -> Option<VehicleId>;
```

### 5.3. Error Propagation

Use `?` operator for error propagation:

```rust
pub fn process_vehicle(path: &Path) -> Result<Vehicle> {
    let raw_json = std::fs::read_to_string(path)?;
    let vehicle: Vehicle = serde_json::from_str(&raw_json)?;
    vehicle.validate()?;
    Ok(vehicle)
}
```

### 5.4. Never Panic in Libraries

Libraries (ev-core) should NEVER panic. Applications can panic for unrecoverable errors.

```rust
// Library code - NO PANIC
pub fn parse_year(s: &str) -> Result<Year, ParseError> {
    let value: u16 = s.parse()
        .map_err(|_| ParseError::InvalidFormat)?;
    Year::new(value)
}

// Application code - panic is acceptable for truly unrecoverable errors
pub fn main() {
    let config = load_config()
        .expect("Configuration file is required");
}
```

---

## 6. Architectural Patterns

### 6.1. Port and Adapter Pattern

Define traits (ports) in ev-core, implement them (adapters) in outer crates.

**ev-core (port definition):**
```rust
// Port - abstract interface
pub trait VehicleRepository {
    fn find_by_id(&self, id: &VehicleId) -> Result<Option<Vehicle>>;
    fn save(&mut self, vehicle: Vehicle) -> Result<VehicleId>;
    fn list_all(&self) -> Result<Vec<Vehicle>>;
}
```

**ev-server (adapter implementation):**
```rust
// Adapter - concrete implementation
pub struct SqliteVehicleRepository {
    connection: rusqlite::Connection,
}

impl VehicleRepository for SqliteVehicleRepository {
    fn find_by_id(&self, id: &VehicleId) -> Result<Option<Vehicle>> {
        // SQLite-specific implementation
    }
    
    fn save(&mut self, vehicle: Vehicle) -> Result<VehicleId> {
        // SQLite-specific implementation
    }
    
    fn list_all(&self) -> Result<Vec<Vehicle>> {
        // SQLite-specific implementation
    }
}
```

### 6.2. Dependency Injection

Use constructor injection with trait bounds:

```rust
// Service depends on abstraction, not implementation
pub struct EtlPipeline<R, G>
where
    R: VehicleRepository,
    G: OutputGenerator,
{
    repository: R,
    generator: G,
}

impl<R, G> EtlPipeline<R, G>
where
    R: VehicleRepository,
    G: OutputGenerator,
{
    pub fn new(repository: R, generator: G) -> Self {
        Self { repository, generator }
    }
    
    pub fn process(&mut self, vehicles: Vec<Vehicle>) -> Result<Vec<u8>> {
        for vehicle in vehicles {
            self.repository.save(vehicle)?;
        }
        let all_vehicles = self.repository.list_all()?;
        self.generator.generate(&all_vehicles)
    }
}
```

### 6.3. Strategy Pattern

Use traits to define interchangeable algorithms:

```rust
pub trait MergeStrategy {
    fn merge(&self, base: &Json, overlay: &Json) -> Json;
}

pub struct DeepMergeStrategy;
impl MergeStrategy for DeepMergeStrategy {
    fn merge(&self, base: &Json, overlay: &Json) -> Json {
        // Deep merge implementation
    }
}

pub struct ShallowMergeStrategy;
impl MergeStrategy for ShallowMergeStrategy {
    fn merge(&self, base: &Json, overlay: &Json) -> Json {
        // Shallow merge implementation
    }
}
```

### 6.4. Repository Pattern

Separate data access from business logic:

```rust
// Repository handles persistence
pub struct VehicleRepositoryImpl {
    db: Database,
}

impl VehicleRepositoryImpl {
    fn to_domain(&self, row: DbRow) -> Result<Vehicle> {
        // Map database row to domain entity
    }
    
    fn from_domain(&self, vehicle: &Vehicle) -> DbRow {
        // Map domain entity to database row
    }
}
```

---

## 7. Async/Await Patterns

### 7.1. When to Use Async

Use async for:
- HTTP requests/responses (ev-server)
- Database connections with async drivers
- Concurrent I/O operations

Don't use async for:
- Pure computation
- ev-core (domain logic should be sync)
- ETL processing (unless parallelizing I/O)

### 7.2. Async Trait Methods

Use `async-trait` crate for trait methods:

```rust
use async_trait::async_trait;

#[async_trait]
pub trait AsyncVehicleRepository {
    async fn find_by_id(&self, id: &VehicleId) -> Result<Option<Vehicle>>;
    async fn save(&self, vehicle: Vehicle) -> Result<VehicleId>;
}

#[async_trait]
impl AsyncVehicleRepository for PostgresRepository {
    async fn find_by_id(&self, id: &VehicleId) -> Result<Option<Vehicle>> {
        // Async implementation
    }
    
    async fn save(&self, vehicle: Vehicle) -> Result<VehicleId> {
        // Async implementation
    }
}
```

### 7.3. Concurrent Processing

Use `tokio::spawn` for true parallelism, `futures::join!` for concurrent awaiting:

```rust
use futures::future::join_all;

pub async fn load_multiple_vehicles(ids: Vec<VehicleId>) -> Result<Vec<Vehicle>> {
    let futures = ids
        .into_iter()
        .map(|id| async move {
            load_vehicle(&id).await
        });
    
    join_all(futures).await
        .into_iter()
        .collect::<Result<Vec<_>>>()
}
```

### 7.4. Tokio Runtime Selection

```rust
// For servers - multi-threaded runtime
#[tokio::main]
async fn main() -> Result<()> {
    // Server code
}

// For CLIs - current thread runtime (lighter)
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    // CLI code
}
```

---

## 8. Testing Strategy

### 8.1. Test Organization

```
src/
  vehicle.rs
  vehicle_repository.rs

tests/                          # Integration tests
  vehicle_integration_test.rs
  
benches/                        # Benchmarks
  vehicle_benchmark.rs
```

### 8.2. Unit Tests

Place unit tests in the same file as the code:

```rust
pub struct Vehicle {
    // ...
}

impl Vehicle {
    pub fn is_available_in_year(&self, year: u16) -> bool {
        // implementation
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vehicle_availability() {
        let vehicle = Vehicle::new(/* ... */);
        assert!(vehicle.is_available_in_year(2024));
        assert!(!vehicle.is_available_in_year(1999));
    }
    
    #[test]
    fn test_invalid_year_returns_error() {
        let result = Year::new(3000);
        assert!(result.is_err());
    }
}
```

### 8.3. Integration Tests

Place integration tests in `tests/` directory:

```rust
// tests/etl_pipeline_test.rs
use ev_etl::EtlPipeline;
use ev_core::Vehicle;

#[test]
fn test_full_etl_pipeline() {
    let input_dir = "fixtures/sample_vehicles";
    let output_dir = temp_dir();
    
    let result = EtlPipeline::new()
        .input(input_dir)
        .output(output_dir)
        .formats(vec!["json", "sqlite"])
        .run();
    
    assert!(result.is_ok());
    assert!(output_dir.join("vehicles.json").exists());
    assert!(output_dir.join("vehicles.db").exists());
}
```

### 8.4. Property-Based Testing

Use `proptest` for property-based testing:

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_year_roundtrip(year in 1900u16..=2100u16) {
        let y = Year::new(year).unwrap();
        let json = serde_json::to_string(&y).unwrap();
        let parsed: Year = serde_json::from_str(&json).unwrap();
        prop_assert_eq!(y, parsed);
    }
}
```

### 8.5. Mocking

Use `mockall` for mocking traits:

```rust
use mockall::{automock, predicate::*};

#[automock]
pub trait VehicleRepository {
    fn find_by_id(&self, id: &VehicleId) -> Result<Option<Vehicle>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_with_mock_repository() {
        let mut mock_repo = MockVehicleRepository::new();
        mock_repo
            .expect_find_by_id()
            .with(eq(VehicleId::new("test".to_string()).unwrap()))
            .times(1)
            .returning(|_| Ok(Some(create_test_vehicle())));
        
        let service = VehicleService::new(mock_repo);
        let result = service.get_vehicle_details(&VehicleId::new("test".to_string()).unwrap());
        assert!(result.is_ok());
    }
}
```

### 8.6. Test Utilities

Create test helpers in a common module:

```rust
// tests/common/mod.rs
#![allow(dead_code)]

use ev_core::*;

pub fn create_test_vehicle() -> Vehicle {
    VehicleBuilder::new()
        .make(SlugName::new("test_make", "Test Make").unwrap())
        .model(SlugName::new("test_model", "Test Model").unwrap())
        .year(Year::new(2024).unwrap())
        .battery(create_test_battery())
        .build()
        .unwrap()
}

pub fn create_test_battery() -> BatterySpecs {
    BatterySpecs {
        pack_capacity_kwh_net: Some(60.0),
        thermal_management: Some(ThermalManagement::Liquid),
        ..Default::default()
    }
}

// tests/vehicle_test.rs
mod common;

#[test]
fn test_something() {
    let vehicle = common::create_test_vehicle();
    // ...
}
```

---

## 9. Performance Guidelines

### 9.1. Avoid Unnecessary Allocations

```rust
// Good - borrows instead of cloning
pub fn process_vehicles(vehicles: &[Vehicle]) -> Vec<String> {
    vehicles.iter()
        .map(|v| format_vehicle(v))
        .collect()
}

// Bad - unnecessary clone
pub fn process_vehicles(vehicles: &[Vehicle]) -> Vec<String> {
    vehicles.to_vec()  // Clones all vehicles!
        .iter()
        .map(|v| format_vehicle(v))
        .collect()
}
```

### 9.2. Use Iterators Instead of Loops

```rust
// Good - iterator chain
let total_capacity: f64 = vehicles
    .iter()
    .filter(|v| v.year == 2024)
    .filter_map(|v| v.battery.pack_capacity_kwh_net)
    .sum();

// Bad - manual loop
let mut total_capacity = 0.0;
for vehicle in vehicles {
    if vehicle.year == 2024 {
        if let Some(capacity) = vehicle.battery.pack_capacity_kwh_net {
            total_capacity += capacity;
        }
    }
}
```

### 9.3. Prefer `&str` Over `String` in Function Parameters

```rust
// Good - accepts both String and &str
pub fn parse_vehicle_id(id: &str) -> Result<VehicleId> {
    VehicleId::new(id.to_string())
}

// Bad - forces caller to own a String
pub fn parse_vehicle_id(id: String) -> Result<VehicleId> {
    VehicleId::new(id)
}
```

### 9.4. Use `Cow` for Conditional Ownership

```rust
use std::borrow::Cow;

pub fn normalize_slug(s: &str) -> Cow<str> {
    if s.chars().all(|c| c.is_ascii_lowercase() || c == '_') {
        Cow::Borrowed(s)  // No allocation needed
    } else {
        Cow::Owned(s.to_lowercase().replace('-', "_"))  // Allocate only if needed
    }
}
```

### 9.5. Parallel Processing with Rayon

For CPU-bound work, use `rayon`:

```rust
use rayon::prelude::*;

pub fn validate_all_vehicles(vehicles: Vec<Vehicle>) -> Vec<Result<(), ValidationError>> {
    vehicles
        .par_iter()  // Parallel iterator
        .map(|v| v.validate())
        .collect()
}
```

### 9.6. Benchmarking

Use `criterion` for benchmarking:

```rust
// benches/vehicle_benchmark.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ev_core::Vehicle;

fn benchmark_vehicle_validation(c: &mut Criterion) {
    let vehicle = create_test_vehicle();
    
    c.bench_function("validate_vehicle", |b| {
        b.iter(|| {
            vehicle.validate()
        });
    });
}

criterion_group!(benches, benchmark_vehicle_validation);
criterion_main!(benches);
```

---

## 10. Security Practices

### 10.1. Input Validation

Always validate untrusted input:

```rust
pub fn load_vehicle_from_file(path: &Path) -> Result<Vehicle> {
    // Validate path is within expected directory
    let canonical_path = path.canonicalize()?;
    if !canonical_path.starts_with("/allowed/directory") {
        return Err(SecurityError::InvalidPath.into());
    }
    
    // Limit file size
    let metadata = std::fs::metadata(&canonical_path)?;
    if metadata.len() > MAX_FILE_SIZE {
        return Err(SecurityError::FileTooLarge.into());
    }
    
    // Parse with size limits
    let content = std::fs::read_to_string(&canonical_path)?;
    serde_json::from_str(&content)
        .map_err(|e| e.into())
}
```

### 10.2. Avoid SQL Injection

Use parameterized queries:

```rust
// Good - parameterized
pub fn find_vehicles_by_make(&self, make: &str) -> Result<Vec<Vehicle>> {
    let mut stmt = self.conn.prepare(
        "SELECT * FROM vehicles WHERE make_slug = ?1"
    )?;
    
    stmt.query_map([make], |row| {
        // Map row to Vehicle
    })
}

// Bad - string concatenation (SQL injection risk)
pub fn find_vehicles_by_make(&self, make: &str) -> Result<Vec<Vehicle>> {
    let query = format!("SELECT * FROM vehicles WHERE make_slug = '{}'", make);
    // NEVER DO THIS
}
```

### 10.3. Secrets Management

Never hardcode secrets:

```rust
// Good - environment variable
pub fn database_url() -> Result<String> {
    std::env::var("DATABASE_URL")
        .context("DATABASE_URL environment variable not set")
}

// Bad - hardcoded
pub fn database_url() -> String {
    "postgresql://user:password123@localhost/db".to_string()
}
```

### 10.4. Dependencies

- Review dependencies before adding
- Use `cargo audit` regularly
- Pin dependency versions in `Cargo.lock`
- Prefer well-maintained crates

---

## 11. Documentation Standards

### 11.1. Module Documentation

Every module should have a doc comment:

```rust
//! Vehicle repository implementations.
//!
//! This module provides database access for vehicle entities through
//! implementations of the `VehicleRepository` trait.
//!
//! # Examples
//!
//! ```
//! use ev_core::{Vehicle, VehicleRepository};
//!
//! let repo = SqliteVehicleRepository::new("vehicles.db")?;
//! let vehicle = repo.find_by_id(&id)?;
//! ```

pub mod sqlite;
pub mod postgresql;
```

### 11.2. Type Documentation

Document all public types:

```rust
/// Represents an electric vehicle with complete specifications.
///
/// A `Vehicle` contains all the essential information about an EV including
/// manufacturer details, battery specifications, charging capabilities, and
/// performance metrics.
///
/// # Examples
///
/// ```
/// use ev_core::{Vehicle, VehicleBuilder};
///
/// let vehicle = VehicleBuilder::new()
///     .make("tesla", "Tesla")
///     .model("model_3", "Model 3")
///     .year(2024)
///     .build()?;
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vehicle {
    /// Vehicle manufacturer information
    pub make: SlugName,
    
    /// Vehicle model name and slug
    pub model: SlugName,
    
    /// Model year (1900-2100)
    pub year: Year,
    
    /// Battery specifications and capabilities
    pub battery: BatterySpecs,
}
```

### 11.3. Function Documentation

Document public functions with examples:

```rust
/// Validates a vehicle against the JSON schema and business rules.
///
/// # Errors
///
/// Returns `ValidationError` if:
/// - Required fields are missing
/// - Values are out of valid range
/// - Business rules are violated
///
/// # Examples
///
/// ```
/// let vehicle = create_test_vehicle();
/// assert!(validate_vehicle(&vehicle).is_ok());
/// ```
pub fn validate_vehicle(vehicle: &Vehicle) -> Result<(), ValidationError> {
    // Implementation
}
```

### 11.4. Error Documentation

Document error variants:

```rust
/// Errors that can occur during vehicle validation.
#[derive(Debug, Error)]
pub enum ValidationError {
    /// A required field is missing from the vehicle data.
    ///
    /// This typically indicates incomplete input data.
    #[error("Missing required field: {field}")]
    MissingField {
        /// The name of the missing field
        field: String,
    },
    
    /// The year value is outside the valid range (1900-2100).
    #[error("Invalid year: {0}. Must be between 1900 and 2100")]
    InvalidYear(u16),
}
```

### 11.5. README per Crate

Each crate should have a README.md:

````markdown
# ev-core

Core domain types and validation logic for OpenEV Data.

## Features

- Type-safe domain entities
- Comprehensive validation
- Zero I/O dependencies

## Usage

```rust
use ev_core::{Vehicle, VehicleBuilder};

let vehicle = VehicleBuilder::new()
    .make("tesla", "Tesla")
    .build()?;
```

## Architecture

This crate forms the Core of the Hexagonal Architecture and has no
dependencies on I/O, HTTP, or database libraries.
````

---

## 12. Code Quality Tools

### 12.1. Rustfmt Configuration

Create `.rustfmt.toml`:

```toml
edition = "2024"
max_width = 100
tab_spaces = 4
newline_style = "Unix"
use_small_heuristics = "Default"
imports_granularity = "Crate"
group_imports = "StdExternalCrate"
```

### 12.2. Clippy Configuration

Create `.clippy.toml`:

```toml
cognitive-complexity-threshold = 30
```

Run with strict lints:

```bash
cargo clippy --all-features -- -D warnings \
    -W clippy::pedantic \
    -W clippy::nursery \
    -W clippy::unwrap_used \
    -W clippy::expect_used
```

### 12.3. Allowed Lints (When Necessary)

```rust
// At crate level for specific exceptions
#![allow(clippy::module_name_repetitions)]  // Vehicle in VehicleRepository

// At item level for specific cases
#[allow(clippy::too_many_arguments)]  // Rare, justified cases
pub fn complex_function(/* many params */) {}
```

### 12.4. CI Quality Gates

All code must pass:

```bash
# Format check
cargo fmt --all -- --check

# Linting
cargo clippy --all-targets --all-features -- -D warnings

# Tests
cargo test --all-features

# Documentation
cargo doc --no-deps --all-features

# Audit
cargo audit
```

---

## 13. Common Patterns

### 13.1. Configuration Management

```rust
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub port: u16,
    #[serde(default = "default_log_level")]
    pub log_level: String,
}

fn default_log_level() -> String {
    "info".to_string()
}

impl Config {
    pub fn from_env() -> Result<Self> {
        envy::from_env().context("Failed to load configuration from environment")
    }
}
```

### 13.2. Logging

Use `tracing` for structured logging:

```rust
use tracing::{info, warn, error, debug, instrument};

#[instrument(skip(repository))]
pub async fn load_vehicle(
    id: &VehicleId,
    repository: &impl VehicleRepository,
) -> Result<Vehicle> {
    debug!("Loading vehicle with id: {}", id);
    
    let vehicle = repository.find_by_id(id).await?;
    
    match vehicle {
        Some(v) => {
            info!("Successfully loaded vehicle: {} {}", v.make, v.model);
            Ok(v)
        }
        None => {
            warn!("Vehicle not found: {}", id);
            Err(VehicleError::NotFound)
        }
    }
}
```

### 13.3. Extension Traits

```rust
// Extend external types with project-specific behavior
pub trait VehicleExt {
    fn display_name(&self) -> String;
    fn is_recent(&self) -> bool;
}

impl VehicleExt for Vehicle {
    fn display_name(&self) -> String {
        format!("{} {} {}", self.year, self.make.name, self.model.name)
    }
    
    fn is_recent(&self) -> bool {
        let current_year = chrono::Utc::now().year() as u16;
        self.year.0 >= current_year - 3
    }
}
```

### 13.4. Type State Pattern

```rust
// Different states represented by types
pub struct Unvalidated;
pub struct Validated;

pub struct Vehicle<S> {
    data: VehicleData,
    _state: PhantomData<S>,
}

impl Vehicle<Unvalidated> {
    pub fn new(data: VehicleData) -> Self {
        Self { data, _state: PhantomData }
    }
    
    pub fn validate(self) -> Result<Vehicle<Validated>, ValidationError> {
        // Validation logic
        Ok(Vehicle { data: self.data, _state: PhantomData })
    }
}

impl Vehicle<Validated> {
    // Only validated vehicles can be saved
    pub fn save(&self, repo: &mut impl VehicleRepository) -> Result<()> {
        repo.save(&self.data)
    }
}
```

---

## 14. Anti-Patterns to Avoid

### 14.1. Stringly-Typed Code

```rust
// Bad - relying on strings
fn get_vehicle_field(vehicle: &Vehicle, field: &str) -> Option<String> {
    match field {
        "make" => Some(vehicle.make.clone()),
        "model" => Some(vehicle.model.clone()),
        _ => None,
    }
}

// Good - use enums
enum VehicleField {
    Make,
    Model,
    Year,
}

fn get_vehicle_field(vehicle: &Vehicle, field: VehicleField) -> String {
    match field {
        VehicleField::Make => vehicle.make.clone(),
        VehicleField::Model => vehicle.model.clone(),
        VehicleField::Year => vehicle.year.to_string(),
    }
}
```

### 14.2. Overusing `clone()`

```rust
// Bad - unnecessary clone
pub fn format_vehicles(vehicles: &[Vehicle]) -> Vec<String> {
    vehicles.iter()
        .map(|v| v.clone())  // Unnecessary!
        .map(|v| format!("{} {}", v.make, v.model))
        .collect()
}

// Good - just borrow
pub fn format_vehicles(vehicles: &[Vehicle]) -> Vec<String> {
    vehicles.iter()
        .map(|v| format!("{} {}", v.make, v.model))
        .collect()
}
```

### 14.3. God Objects

```rust
// Bad - one type does everything
pub struct VehicleManager {
    pub data: Vehicle,
    pub db_connection: Connection,
    pub http_client: Client,
    pub cache: Cache,
    // ... more responsibilities
}

// Good - separate concerns
pub struct Vehicle { /* data only */ }
pub struct VehicleRepository { /* persistence */ }
pub struct VehicleService { /* business logic */ }
pub struct VehicleCache { /* caching */ }
```

### 14.4. Ignoring Results

```rust
// Bad - silently ignoring errors
let _ = vehicle.validate();  // Error ignored!

// Good - handle or propagate
vehicle.validate()?;

// Or explicitly acknowledge
let _ = vehicle.validate()
    .map_err(|e| eprintln!("Validation warning: {}", e));
```

### 14.5. Using `unwrap()` in Production Code

```rust
// Bad - can panic
let vehicle = repository.find_by_id(&id).unwrap();

// Good - handle error
let vehicle = repository.find_by_id(&id)?;

// Or provide context
let vehicle = repository.find_by_id(&id)
    .expect("Vehicle must exist at this point");  // Only if truly infallible
```

---

## Summary

Following these guidelines ensures:

1. **Correctness**: Type safety prevents invalid states
2. **Maintainability**: Clear separation of concerns and consistent patterns
3. **Performance**: Efficient use of Rust's zero-cost abstractions
4. **Security**: Proper input validation and secure practices
5. **Testability**: Dependency injection and trait-based design
6. **Documentation**: Self-documenting code with comprehensive docs

When in doubt, prioritize:
1. **Correctness** over performance
2. **Clarity** over cleverness
3. **Simplicity** over optimization

All code submitted to the project will be reviewed against these guidelines.

---

**Last Updated**: 2025-12-25

