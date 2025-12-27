# Testing Guide

This guide covers test organization, execution, and best practices for the OpenEV Data API project.

---

## Test Structure

```
tests/
├── unit/                    # Unit tests mirroring source structure
│   ├── ev_core/
│   │   ├── domain/
│   │   │   ├── battery_test.rs
│   │   │   ├── body_test.rs
│   │   │   ├── charging_test.rs
│   │   │   ├── enums_test.rs
│   │   │   ├── metadata_test.rs
│   │   │   ├── powertrain_test.rs
│   │   │   ├── range_test.rs
│   │   │   ├── sources_test.rs
│   │   │   ├── types_test.rs
│   │   │   └── vehicle_test.rs
│   │   ├── error_test.rs
│   │   └── validation_test.rs
│   ├── ev_etl/
│   │   ├── ingest/
│   │   │   ├── parser_test.rs
│   │   │   └── reader_test.rs
│   │   ├── merge/
│   │   │   └── strategy_test.rs
│   │   ├── output/
│   │   │   ├── csv_test.rs
│   │   │   ├── json_test.rs
│   │   │   ├── postgresql_test.rs
│   │   │   ├── sqlite_test.rs
│   │   │   ├── statistics_test.rs
│   │   │   └── xml_test.rs
│   │   ├── cli_test.rs
│   │   ├── error_test.rs
│   │   └── validate_test.rs
│   └── ev_server/
│       ├── api/
│       │   ├── health_test.rs
│       │   ├── makes_test.rs
│       │   ├── search_test.rs
│       │   └── vehicles_test.rs
│       ├── db/
│       │   └── sqlite_test.rs
│       ├── config_test.rs
│       ├── error_test.rs
│       └── models_test.rs
├── e2e/                     # End-to-end API tests (one file per endpoint)
│   ├── health/
│   │   └── health_check_test.rs
│   ├── makes/
│   │   ├── list_makes_test.rs
│   │   └── list_models_test.rs
│   ├── search/
│   │   └── search_vehicles_test.rs
│   └── vehicles/
│       ├── list_vehicles_test.rs
│       ├── get_vehicle_test.rs
│       ├── get_vehicle_by_code_test.rs
│       └── get_vehicle_variants_test.rs
├── integration/             # Integration tests (cross-component)
│   ├── etl_pipeline_test.rs
│   ├── database_test.rs
│   └── api_database_test.rs
└── others/                  # Other tests
    └── fixtures/
        └── sample_data.json
```

---

## Test Categories

### Unit Tests (`tests/unit/`)

Test individual functions and modules in isolation.

**Structure:** Mirrors the `crates/` directory structure. Each source file has a corresponding `_test.rs` file.

**Coverage:** 100% of public functions and critical private functions.

**Example:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_vehicle_json() {
        let json = r#"{"make": {"slug": "tesla"}}"#;
        let result = parse_vehicle(json);
        assert!(result.is_ok());
    }
}
```

### E2E Tests (`tests/e2e/`)

Test complete API workflows from request to response.

**Structure:** Organized by endpoint group (`health/`, `makes/`, `search/`, `vehicles/`). Each endpoint has its own test file testing all success and error scenarios.

**Required scenarios per endpoint:**
- ✅ Success response (200)
- ❌ Not Found (404) where applicable
- ❌ Bad Request (400) where applicable
- ❌ Internal Error (500) simulation

**Example:**
```rust
#[tokio::test]
async fn test_get_vehicle_success() {
    let response = client.get("/api/v1/vehicles/tesla/model_3/2024").await;
    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn test_get_vehicle_not_found() {
    let response = client.get("/api/v1/vehicles/invalid/invalid/9999").await;
    assert_eq!(response.status(), 404);
    let body: ProblemDetails = response.json().await;
    assert!(body.error_type.contains("not-found"));
}
```

### Integration Tests (`tests/integration/`)

Test component interactions and data flows.

**Focus:**
- ETL pipeline end-to-end
- Database + API integration
- Cross-crate functionality

### Other Tests (`tests/others/`)

Tests that don't fit other categories:
- Performance benchmarks
- Test fixtures
- Helper utilities

---

## Running Tests

### All Tests

```bash
cargo test --workspace
```

### Specific Category

```bash
# Unit tests only
cargo test --test unit

# E2E tests only
cargo test --test e2e

# Integration tests only
cargo test --test integration
```

### Specific Crate

```bash
cargo test -p ev-core
cargo test -p ev-etl
cargo test -p ev-server
```

### With Output

```bash
cargo test --all -- --nocapture
```

### Watch Mode

```bash
cargo watch -x "test --all"
```

---

## Test Conventions

1. **Naming:** `test_<function_name>_<scenario>`
2. **Assertions:** Use descriptive assertions with messages
3. **Fixtures:** Place in `tests/others/fixtures/`
4. **Database:** Use in-memory SQLite for unit tests
5. **HTTP Client:** Use `reqwest` or test server utilities for e2e

---

## Coverage

Generate coverage report using `cargo-llvm-cov` (Recommended):

```bash
# Install the tool
cargo install cargo-llvm-cov

# Generate text summary
cargo llvm-cov --workspace

# Generate HTML report (Interactive)
cargo llvm-cov --workspace --html
# Report will be at: target/llvm-cov/html/index.html

# Generate LCOV info (for CI)
cargo llvm-cov --workspace --lcov --output-path lcov.info
```

---

## Adding New Tests

1. Create file in appropriate directory following structure
2. Import the module under test
3. Write tests covering success and error cases
4. Run `cargo test` to verify
5. Update this guide if adding new test categories
