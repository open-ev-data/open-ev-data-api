# Getting Started with OpenEV Data API

This guide will help you set up your local development environment and start working with the OpenEV Data API and ETL pipeline.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Repository Setup](#repository-setup)
- [Building the Project](#building-the-project)
- [Running the ETL Pipeline](#running-the-etl-pipeline)
- [Running the API Server](#running-the-api-server)
- [Testing Your Setup](#testing-your-setup)
- [Development Workflow](#development-workflow)
- [Common Tasks](#common-tasks)
- [Troubleshooting](#troubleshooting)

---

## Prerequisites

### Required Software

1. **Rust Toolchain** (1.80.0 or later)
   ```bash
   # Install rustup (Rust installer)
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   
   # Or on Windows, download from: https://rustup.rs/
   
   # Install stable toolchain
   rustup install stable
   rustup default stable
   
   # Verify installation
   rustc --version
   cargo --version
   ```

2. **Git**
   ```bash
   # Verify Git is installed
   git --version
   ```

3. **Optional but Recommended**
   - **SQLite CLI** - For inspecting generated databases
   - **PostgreSQL Client** - For testing PostgreSQL output
   - **curl** or **httpie** - For testing API endpoints
   - **jq** - For pretty-printing JSON responses

### Development Tools (Optional)

```bash
# Rust development tools
cargo install cargo-watch    # Auto-reload on changes
cargo install cargo-edit     # Manage dependencies
cargo install cargo-nextest  # Better test runner

# Database tools
cargo install sqlite-utils    # SQLite utilities
```

---

## Repository Setup

### Directory Structure

We recommend cloning all OpenEV Data repositories side-by-side for easier cross-repository development:

```bash
# Create a parent directory for all OpenEV projects
mkdir -p ~/projects/open-ev-data
cd ~/projects/open-ev-data

# Clone repositories
git clone https://github.com/open-ev-data/open-ev-data-dataset.git
git clone https://github.com/open-ev-data/open-ev-data-api.git
git clone https://github.com/open-ev-data/.github.git

# Your structure should look like:
# ~/projects/open-ev-data/
# ├── open-ev-data-dataset/    # Vehicle data source files
# ├── open-ev-data-api/        # API and ETL (this project)
# └── .github/                 # Project governance
```

**Windows Users:**
```powershell
# Create parent directory
New-Item -ItemType Directory -Path "$HOME\projects\open-ev-data"
cd "$HOME\projects\open-ev-data"

# Clone repositories
git clone https://github.com/open-ev-data/open-ev-data-dataset.git
git clone https://github.com/open-ev-data/open-ev-data-api.git
git clone https://github.com/open-ev-data/.github.git
```

---

## Building the Project

### Initial Build

```bash
cd open-ev-data-api

# Build all workspace crates in debug mode
cargo build --all

# Build in release mode (optimized, slower compile)
cargo build --all --release
```

### Verify Build

```bash
# Run all tests
cargo test --all

# Check code formatting
cargo fmt --all -- --check

# Run linter
cargo clippy --all -- -D warnings
```

---

## Running the ETL Pipeline

The ETL pipeline reads the layered JSON files from the dataset repository and generates output artifacts.

### Basic ETL Execution

```bash
cd open-ev-data-api

# Run ETL pointing to local dataset
cargo run -p ev-etl -- \
  --input ../open-ev-data-dataset/src \
  --output ./output \
  --formats json,sqlite

# This will generate:
# - output/vehicles.json
# - output/vehicles.db
# - output/validation-report.txt
# - output/statistics.json
```

### ETL Command Options

```bash
# Generate all formats
cargo run -p ev-etl -- \
  --input ../open-ev-data-dataset/src \
  --output ./output \
  --formats json,sqlite,postgresql,csv,xml

# Generate only specific format
cargo run -p ev-etl -- \
  --input ../open-ev-data-dataset/src \
  --output ./output \
  --formats json

# Verbose output for debugging
cargo run -p ev-etl -- \
  --input ../open-ev-data-dataset/src \
  --output ./output \
  --formats json,sqlite \
  --verbose

# Validate only (no output generation)
cargo run -p ev-etl -- \
  --input ../open-ev-data-dataset/src \
  --validate-only
```

### Inspecting Generated Artifacts

#### JSON Output
```bash
# View generated JSON
cat output/vehicles.json | jq '.'

# Count vehicles
cat output/vehicles.json | jq '.vehicle_count'

# View first vehicle
cat output/vehicles.json | jq '.vehicles[0]'

# Find specific make
cat output/vehicles.json | jq '.vehicles[] | select(.make.slug == "tesla")'
```

#### SQLite Database
```bash
# Open database
sqlite3 output/vehicles.db

# View tables
.tables

# Count vehicles
SELECT COUNT(*) FROM vehicles;

# Query by make
SELECT make_slug, model_slug, year, trim_slug 
FROM vehicles 
WHERE make_slug = 'tesla';

# Exit
.quit
```

#### PostgreSQL SQL
```bash
# View generated SQL
head -n 50 output/vehicles.sql

# Import into local PostgreSQL (if you have it)
createdb openev_test
psql -d openev_test -f output/vehicles.sql
```

#### CSV Output
```bash
# View CSV headers
head -n 1 output/vehicles.csv

# Count rows
wc -l output/vehicles.csv

# View in spreadsheet
# Open output/vehicles.csv in Excel, LibreOffice, or Google Sheets
```

---

## Running the API Server

The API server serves vehicle data through REST endpoints using the generated database.

### Basic Server Execution

```bash
cd open-ev-data-api

# Run server with SQLite database
cargo run -p ev-server -- \
  --database ./output/vehicles.db \
  --port 3000

# Server will start at http://localhost:3000
```

### Server Configuration Options

```bash
# Custom port
cargo run -p ev-server -- \
  --database ./output/vehicles.db \
  --port 8080

# PostgreSQL database (if available)
cargo run -p ev-server -- \
  --database-url postgresql://user:pass@localhost/openev \
  --port 3000

# Enable debug logging
RUST_LOG=debug cargo run -p ev-server -- \
  --database ./output/vehicles.db \
  --port 3000

# Production mode (release build)
cargo run --release -p ev-server -- \
  --database ./output/vehicles.db \
  --port 3000
```

### Server with Auto-Reload (Development)

```bash
# Install cargo-watch if not already
cargo install cargo-watch

# Run with auto-reload on code changes
cargo watch -x 'run -p ev-server -- --database ./output/vehicles.db --port 3000'
```

---

## Testing Your Setup

### API Health Check

```bash
# Using curl
curl http://localhost:3000/api/v1/health

# Expected response:
# {
#   "status": "healthy",
#   "version": "1.0.0",
#   "database": "connected",
#   "vehicle_count": 123
# }
```

### Test API Endpoints

```bash
# List all manufacturers
curl http://localhost:3000/api/v1/makes | jq '.'

# List vehicles (first page)
curl http://localhost:3000/api/v1/vehicles | jq '.'

# Filter by make
curl "http://localhost:3000/api/v1/vehicles?make=tesla" | jq '.'

# Get specific vehicle
curl http://localhost:3000/api/v1/vehicles/tesla/model_3/2024 | jq '.'

# List models for a make
curl http://localhost:3000/api/v1/makes/tesla/models | jq '.'

# Search vehicles
curl "http://localhost:3000/api/v1/search?q=dolphin" | jq '.'

# Pagination
curl "http://localhost:3000/api/v1/vehicles?page=2&per_page=10" | jq '.'
```

### Using HTTPie (Alternative)

```bash
# Install httpie (optional)
pip install httpie

# Test endpoints with prettier output
http :3000/api/v1/health
http :3000/api/v1/makes
http :3000/api/v1/vehicles make==tesla
http :3000/api/v1/vehicles/byd/dolphin/2024
```

### OpenAPI Documentation

Once the server is running with OpenAPI support:

```bash
# Access Swagger UI
open http://localhost:3000/swagger-ui/

# Download OpenAPI spec
curl http://localhost:3000/api-docs/openapi.json > openapi.json
```

---

## Development Workflow

### Typical Development Cycle

1. **Make changes to dataset** (add/modify vehicles in `open-ev-data-dataset/src`)

2. **Regenerate artifacts**
   ```bash
   cd open-ev-data-api
   cargo run -p ev-etl -- \
     --input ../open-ev-data-dataset/src \
     --output ./output \
     --formats json,sqlite
   ```

3. **Restart API server** (or use cargo-watch for auto-reload)
   ```bash
   cargo run -p ev-server -- --database ./output/vehicles.db --port 3000
   ```

4. **Test changes**
   ```bash
   curl http://localhost:3000/api/v1/vehicles/your/new/vehicle | jq '.'
   ```

### Working on ETL Code

```bash
# Make changes to ev-etl or ev-core

# Run unit tests
cargo test -p ev-etl
cargo test -p ev-core

# Test with real dataset
cargo run -p ev-etl -- \
  --input ../open-ev-data-dataset/src \
  --output ./test-output \
  --formats json

# Inspect results
cat test-output/validation-report.txt
cat test-output/statistics.json | jq '.'
```

### Working on API Server Code

```bash
# Make changes to ev-server

# Run unit tests
cargo test -p ev-server

# Run server with hot reload
cargo watch -x 'run -p ev-server -- --database ./output/vehicles.db --port 3000'

# In another terminal, test endpoints
curl http://localhost:3000/api/v1/health
```

### Running Integration Tests

```bash
# Run all integration tests
cargo test --test '*'

# Run specific integration test
cargo test --test etl_pipeline_test

# Run with output
cargo test --test api_integration_test -- --nocapture
```

---

## Common Tasks

### Task: Add a New Vehicle to Dataset

1. **Add vehicle JSON** in `open-ev-data-dataset/src/`
   ```bash
   cd open-ev-data-dataset
   mkdir -p src/newmake/newmodel/2024
   
   # Create base.json, year base, and variants
   ```

2. **Regenerate artifacts**
   ```bash
   cd ../open-ev-data-api
   cargo run -p ev-etl -- \
     --input ../open-ev-data-dataset/src \
     --output ./output \
     --formats json,sqlite
   ```

3. **Verify in database**
   ```bash
   sqlite3 output/vehicles.db "SELECT * FROM vehicles WHERE make_slug='newmake';"
   ```

4. **Test via API**
   ```bash
   cargo run -p ev-server -- --database ./output/vehicles.db --port 3000
   curl http://localhost:3000/api/v1/vehicles/newmake/newmodel/2024 | jq '.'
   ```

### Task: Test ETL Merge Logic

```bash
# Create test fixtures in tests/fixtures/

# Run merge-specific tests
cargo test -p ev-etl merge

# Test with minimal dataset
cargo run -p ev-etl -- \
  --input ./tests/fixtures/sample_vehicles \
  --output ./test-output \
  --formats json \
  --verbose
```

### Task: Validate Dataset Changes

```bash
# Validate without generating artifacts
cargo run -p ev-etl -- \
  --input ../open-ev-data-dataset/src \
  --validate-only

# View validation report
cat validation-report.txt
```

### Task: Benchmark Performance

```bash
# Run benchmarks
cargo bench

# Benchmark ETL only
cargo bench -p ev-etl

# Benchmark API only
cargo bench -p ev-server
```

### Task: Generate Release Artifacts

```bash
# Build optimized binaries
cargo build --release --all

# Generated binaries are in:
ls -lh target/release/ev-etl
ls -lh target/release/ev-server

# Generate production artifacts
./target/release/ev-etl \
  --input ../open-ev-data-dataset/src \
  --output ./release \
  --formats json,sqlite,postgresql,csv,xml

# Verify artifacts
ls -lh release/
```

---

## Troubleshooting

### ETL Issues

#### Problem: "Cannot find dataset directory"

```bash
# Check path is correct
ls -la ../open-ev-data-dataset/src

# Use absolute path if needed
cargo run -p ev-etl -- \
  --input ~/projects/open-ev-data/open-ev-data-dataset/src \
  --output ./output \
  --formats json
```

#### Problem: "Validation errors"

```bash
# Run with verbose output
cargo run -p ev-etl -- \
  --input ../open-ev-data-dataset/src \
  --validate-only \
  --verbose

# Check validation report
cat validation-report.txt
```

#### Problem: "Out of memory during ETL"

```bash
# Process in smaller batches (future feature)
# For now, try release mode for better memory usage
cargo run --release -p ev-etl -- \
  --input ../open-ev-data-dataset/src \
  --output ./output \
  --formats json,sqlite
```

### API Server Issues

#### Problem: "Cannot open database file"

```bash
# Verify database exists
ls -la ./output/vehicles.db

# Check file permissions
chmod 644 ./output/vehicles.db

# Use absolute path
cargo run -p ev-server -- \
  --database "$(pwd)/output/vehicles.db" \
  --port 3000
```

#### Problem: "Port already in use"

```bash
# Find process using port
lsof -i :3000  # macOS/Linux
netstat -ano | findstr :3000  # Windows

# Kill process or use different port
cargo run -p ev-server -- \
  --database ./output/vehicles.db \
  --port 8080
```

#### Problem: "Database is locked"

```bash
# Close any other connections to the database
# Make sure no SQLite CLI sessions are open

# Restart server
```

### Build Issues

#### Problem: "Cannot compile - missing dependencies"

```bash
# Update Rust
rustup update stable

# Clean and rebuild
cargo clean
cargo build --all
```

#### Problem: "Clippy errors"

```bash
# Auto-fix what's possible
cargo clippy --fix --all

# Check remaining issues
cargo clippy --all -- -D warnings
```

#### Problem: "Tests failing"

```bash
# Run tests with output
cargo test --all -- --nocapture

# Run specific test
cargo test test_name -- --nocapture

# Update test fixtures if needed
```

### Cross-Repository Issues

#### Problem: "Dataset changes not reflected in API"

```bash
# Ensure you regenerated artifacts after dataset changes
cd open-ev-data-api
cargo run -p ev-etl -- \
  --input ../open-ev-data-dataset/src \
  --output ./output \
  --formats sqlite

# Restart API server
```

#### Problem: "Schema mismatch between dataset and API"

```bash
# Check schema versions match
cat ../open-ev-data-dataset/schema.json | jq '.properties.schema_version'
cargo run -p ev-core --bin schema_version

# Ensure API is synced with latest dataset schema
cd open-ev-data-api
git pull origin main
cargo build --all
```

---

## Getting Help

- **Documentation**: Check [ARCHITECTURE.md](./docs/ARCHITECTURE.md) for detailed design
- **Issues**: Report bugs at [GitHub Issues](https://github.com/open-ev-data/open-ev-data-api/issues)
- **Discussions**: Ask questions at [GitHub Discussions](https://github.com/open-ev-data/.github/discussions)
- **Contributing**: See [CONTRIBUTING.md](https://github.com/open-ev-data/.github/CONTRIBUTING.md)

---

## Next Steps

Once you're comfortable with the basics:

1. Read [ARCHITECTURE.md](./docs/ARCHITECTURE.md) for deeper understanding
2. Review the [TODO.md](https://github.com/open-ev-data/.github/TODO.md) to see what's being worked on
3. Pick a task from the TODO and contribute!
4. Check out the test suite to understand expected behavior
5. Explore deployment options (Docker, PostgreSQL, etc.)

---

**Happy Coding!** 🚗⚡

