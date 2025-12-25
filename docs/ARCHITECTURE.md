# Architecture Reference: OpenEV Data API

## 1. Introduction & Architectural Style

The **OpenEV Data API** is engineered using **Hexagonal Architecture (Ports and Adapters)**, implemented via a **Rust Cargo Workspace**. This architectural choice strictly enforces the **Dependency Rule**, ensuring that the business logic (Core) remains independent of frameworks, databases, and external interfaces.

### 1.1. System Objective

The system serves two primary objectives:

1. **Data Processing Pipeline (ETL)**: Transform the layered JSON dataset into multiple ready-to-consume formats:
   - Complete canonical JSON (all vehicles expanded and validated)
   - SQLite database (embedded, ready-to-query)
   - PostgreSQL schema + data (production-ready)
   - CSV export (analysis and spreadsheet integration)
   - XML export (legacy system compatibility)

2. **API Server**: Provide a high-performance REST API to query vehicle data, deployable in multiple environments (containers, serverless, edge).

The system prioritizes **correctness** (via strict typing), **reproducibility** (deterministic builds), and **portability** (multiple output formats and deployment targets).

### 1.2. Build-Time vs Runtime Strategy

**Build-Time (CI/CD Pipeline):**
- Dataset compilation happens during the release process
- Validation and merge logic executed once
- Multiple output artifacts generated and attached to releases
- Ensures data quality before distribution

**Runtime (API Server):**
- Serves pre-compiled, validated data
- No runtime validation overhead
- Optimized for read performance
- Stateless and horizontally scalable

### 1.3. Component Roles (Crates)

The workspace is divided into three distinct crates, each representing a specific architectural layer:

* **`crates/ev-core` (The Core / Inner Hexagon):**
    * **Role:** Pure Domain Library
    * **Responsibility:** Defines domain types (`Vehicle`, `Battery`, `Charging`, etc.), validation rules, and business logic. Contains no I/O, HTTP, or Database code. Shared dependency for all other crates.
    * **Principle:** "Make invalid states unrepresentable"
    * **Key Features:**
        - Rust structs mirroring the JSON schema
        - Serde serialization/deserialization
        - Type-safe enums for classifications
        - Validation logic for data integrity
        - Schema generation support

* **`crates/ev-etl` (Data Processing Pipeline):**
    * **Role:** CLI Tool for Batch Processing
    * **Responsibility:** Executes during CI/CD to transform source data into distributable artifacts
    * **Key Features:**
        - Reads layered JSON files from dataset repository
        - Implements deep merge strategy (base.json → year → variant)
        - Validates against JSON schema and domain types
        - Generates multiple output formats:
            - `vehicles.json` - Complete canonical dataset
            - `vehicles.db` - SQLite database with full schema
            - `vehicles.sql` - PostgreSQL DDL + INSERT statements
            - `vehicles.csv` - Flattened tabular export
            - `vehicles.xml` - XML representation
        - Provides validation reports and statistics

* **`crates/ev-server` (API Server):**
    * **Role:** HTTP REST API Server
    * **Responsibility:** Exposes vehicle data through HTTP endpoints
    * **Key Features:**
        - RESTful API design
        - Query by make, model, year, variant
        - Search and filter capabilities
        - OpenAPI/Swagger documentation
        - Multiple deployment targets:
            - Standalone binary (Linux/Windows/macOS)
            - Docker container
            - Kubernetes deployment
            - Serverless functions (future)
            - Edge workers (future)

### 1.4. Architectural Principles

1. **Dependency Inversion:** High-level modules (Adapters) depend on low-level modules (Core). The Core depends on nothing.
2. **Single Source of Truth:** The dataset repository is the canonical source; all artifacts are derived.
3. **Zero-Cost Abstractions:** Usage of Rust Generics and Traits over runtime polymorphism for compile-time optimization.
4. **Type-Driven Development:** Leverage Rust's type system to prevent invalid states.
5. **Deterministic Builds:** Same input always produces identical output.
6. **Format Agnostic Core:** Core domain logic is independent of serialization formats.

---

## 2. File Directory Tree

This structure represents the physical layout of the Monorepo/Workspace.

```text
open-ev-data-api/
├── .github/
│   ├── workflows/
│   │   ├── ci.yml                # Quality Gates (Clippy, Fmt, Test)
│   │   ├── etl-artifacts.yml     # ETL Pipeline: Build and Attach Artifacts
│   │   └── release.yml           # Semantic Release + Docker Build
│   └── CODEOWNERS                # Governance
├── .cargo/
│   └── config.toml               # Global build flags
├── crates/                       # [WORKSPACE MEMBERS]
│   │
│   ├── ev-core/                  # [LAYER: DOMAIN]
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs            # Library entry point
│   │       ├── domain/           # Domain Entities
│   │       │   ├── mod.rs
│   │       │   ├── vehicle.rs    # Main aggregate root
│   │       │   ├── battery.rs    # Battery specifications
│   │       │   ├── charging.rs   # Charging capabilities
│   │       │   ├── powertrain.rs # Motor and drivetrain
│   │       │   ├── range.rs      # Range and efficiency
│   │       │   └── types.rs      # Common types and enums
│   │       └── validation/       # Validation rules
│   │           └── mod.rs
│   │
│   ├── ev-etl/                   # [LAYER: DATA PROCESSING]
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── main.rs           # CLI entry point
│   │       ├── ingest/           # Data ingestion
│   │       │   ├── mod.rs
│   │       │   ├── reader.rs     # File system reader
│   │       │   └── parser.rs     # JSON parsing
│   │       ├── merge/            # Deep merge logic
│   │       │   ├── mod.rs
│   │       │   └── strategy.rs   # Merge precedence rules
│   │       ├── validate/         # Validation pipeline
│   │       │   └── mod.rs
│   │       └── output/           # Output generators
│   │           ├── mod.rs
│   │           ├── json.rs       # Canonical JSON output
│   │           ├── sqlite.rs     # SQLite database generator
│   │           ├── postgresql.rs # PostgreSQL schema + data
│   │           ├── csv.rs        # CSV export
│   │           └── xml.rs        # XML export
│   │
│   └── ev-server/                # [LAYER: API SERVER]
│       ├── Cargo.toml
│       └── src/
│           ├── main.rs           # Binary entry point
│           ├── config.rs         # Configuration management
│           ├── api/              # HTTP Handlers
│           │   ├── mod.rs
│           │   ├── routes.rs     # Route definitions
│           │   ├── vehicles.rs   # Vehicle endpoints
│           │   ├── search.rs     # Search endpoints
│           │   └── health.rs     # Health check
│           ├── db/               # Database layer
│           │   ├── mod.rs
│           │   ├── sqlite.rs     # SQLite connection
│           │   └── postgresql.rs # PostgreSQL connection (optional)
│           └── models/           # API response models
│               └── mod.rs
│
├── tests/                        # [INTEGRATION TESTS]
│   ├── etl_pipeline_test.rs      # ETL processing tests
│   ├── merge_logic_test.rs       # Merge strategy tests
│   ├── output_format_test.rs     # Output validation tests
│   └── api_integration_test.rs   # API endpoint tests
├── fixtures/                     # [TEST DATA]
│   └── sample_vehicles/          # Sample vehicle data for testing
├── schemas/                      # [DATABASE SCHEMAS]
│   ├── sqlite/
│   │   └── schema.sql            # SQLite schema definition
│   └── postgresql/
│       └── schema.sql            # PostgreSQL schema definition
├── docker/
│   ├── Dockerfile                # API server container
│   └── docker-compose.yml        # Local development setup
├── Cargo.toml                    # Workspace Root Config
├── .releaserc.json               # Semantic Release Config
├── README.md                     # Project Overview
└── docs/
    ├── ARCHITECTURE.md           # This Document
    └── RUST_GUIDELINES.md        # Rust Development Standards

```

---

## 3. System Relationship Diagram

This diagram visualizes the dependencies and data flow boundaries. All dependencies point **inward** toward `ev-core`.

```mermaid
graph TD
    subgraph "Data Sources"
        Dataset[Dataset Repository<br/>Layered JSON Files]
    end

    subgraph "Build-Time (CI/CD)"
        ETL[ev-etl CLI]
        
        subgraph "Output Artifacts"
            JSON[vehicles.json]
            SQLite[vehicles.db]
            PostgreSQL[vehicles.sql]
            CSV[vehicles.csv]
            XML[vehicles.xml]
        end
    end

    subgraph "Runtime (Production)"
        Server[ev-server<br/>REST API]
        DB[(Database<br/>SQLite/PostgreSQL)]
        Client[HTTP Clients]
    end

    subgraph "Core Domain"
        Core[ev-core<br/>Domain Types]
    end

    Dataset -->|Read| ETL
    Core -->|Used by| ETL
    Core -->|Used by| Server
    
    ETL -->|Generate| JSON
    ETL -->|Generate| SQLite
    ETL -->|Generate| PostgreSQL
    ETL -->|Generate| CSV
    ETL -->|Generate| XML
    
    SQLite -.->|Deploy| DB
    PostgreSQL -.->|Deploy| DB
    
    Server -->|Query| DB
    Client -->|HTTP Request| Server
    Server -->|JSON Response| Client

```

### Dependency Flow

1. **ev-core** is the foundation - no dependencies
2. **ev-etl** depends on ev-core for types and validation
3. **ev-server** depends on ev-core for types and serialization
4. **Output artifacts** are standalone - no runtime dependencies on Rust code

---

## 4. Operational Data Flow

This sequence diagram illustrates both the **Build-Time Pipeline** (artifact generation) and the **Runtime Path** (API consumption).

```mermaid
sequenceDiagram
    autonumber
    participant Contributor
    participant DatasetRepo as Dataset Repository
    participant CI as GitHub Actions
    participant ETL as ev-etl CLI
    participant Artifacts as Release Artifacts
    participant Deploy as Deployment
    participant API as ev-server
    participant Client as API Consumer

    note over Contributor, Artifacts: BUILD-TIME PIPELINE
    
    Contributor->>DatasetRepo: Commit JSON changes
    DatasetRepo->>CI: Trigger workflow
    CI->>ETL: Run cargo build --release -p ev-etl
    ETL->>ETL: Load layered JSON files
    ETL->>ETL: Deep merge (base → year → variant)
    ETL->>ETL: Validate against schema
    ETL->>Artifacts: Generate vehicles.json
    ETL->>Artifacts: Generate vehicles.db (SQLite)
    ETL->>Artifacts: Generate vehicles.sql (PostgreSQL)
    ETL->>Artifacts: Generate vehicles.csv
    ETL->>Artifacts: Generate vehicles.xml
    CI->>DatasetRepo: Attach artifacts to release
    
    note over Deploy, Client: RUNTIME PATH
    
    Deploy->>API: Deploy ev-server + vehicles.db
    API->>API: Load database into memory
    Client->>API: GET /api/v1/vehicles?make=tesla
    API->>API: Query local database
    API-->>Client: Return JSON response
    
    Client->>API: GET /api/v1/vehicles/tesla/model_3/2024
    API->>API: Query by composite key
    API-->>Client: Return vehicle details

```

### Pipeline Stages

#### Build-Time (CI/CD)
1. **Trigger**: Dataset repository receives commits
2. **Compilation**: ETL reads and merges layered JSON
3. **Validation**: Schema validation + business rules
4. **Generation**: Multiple output formats created
5. **Distribution**: Artifacts attached to GitHub releases

#### Runtime (API Server)
1. **Initialization**: Server loads pre-built database
2. **Request Handling**: REST endpoints process queries
3. **Query Execution**: Fast lookups in local database
4. **Response**: JSON serialization and delivery

---

## 5. Technology Stack & Versioning

The project utilizes the **Rust 2024 Edition**. Below is the technology stack for the ecosystem.

### 5.1. Language Environment

* **Language:** Rust
* **Edition:** `2024` (Latest Stable Edition)
* **Toolchain:** `stable` (Version 1.85+)
* **MSRV:** 1.80.0 (Minimum Supported Rust Version)

### 5.2. Core Libraries

| Crate | Version | Usage |
| --- | --- | --- |
| **serde** | `1.0` | Serialization/deserialization framework |
| **serde_json** | `1.0` | JSON parsing and generation |
| **anyhow** | `1.0` | Application-level error handling (ETL, Server) |
| **thiserror** | `2.0` | Library-level error definitions (ev-core) |

### 5.3. ETL-Specific Libraries

| Crate | Version | Usage |
| --- | --- | --- |
| **walkdir** | `2.5+` | Recursive directory traversal |
| **jsonschema** | `0.25+` | JSON schema validation |
| **rusqlite** | `0.34+` | SQLite database generation |
| **postgres** | `0.19+` | PostgreSQL SQL generation |
| **csv** | `1.3+` | CSV serialization |
| **quick-xml** | `0.37+` | XML serialization |
| **rayon** | `1.10+` | Parallel processing of vehicle files |

### 5.4. Server-Specific Libraries

| Crate | Version | Usage |
| --- | --- | --- |
| **axum** | `0.8+` | Web framework for REST API |
| **tokio** | `1.42+` | Async runtime |
| **tower** | `0.5+` | Middleware and service abstractions |
| **tower-http** | `0.6+` | HTTP middleware (CORS, compression, tracing) |
| **rusqlite** | `0.34+` | SQLite query layer |
| **sqlx** | `0.8+` | PostgreSQL async query layer (optional) |
| **utoipa** | `5.3+` | OpenAPI documentation generation |
| **tracing** | `0.1+` | Structured logging |
| **tracing-subscriber** | `0.3+` | Log collection and formatting |

### 5.5. Development & Testing

| Crate | Version | Usage |
| --- | --- | --- |
| **criterion** | `0.5+` | Benchmarking |
| **proptest** | `1.6+` | Property-based testing |
| **tempfile** | `3.14+` | Temporary file creation for tests |
| **mockall** | `0.13+` | Mocking framework |

### 5.6. Infrastructure & Deployment

* **Container Runtime:** Docker 27.0+
* **Container Orchestration:** Kubernetes 1.30+ (optional)
* **CI/CD:** GitHub Actions
* **Release Automation:** semantic-release
* **Database (Production):** 
  - SQLite 3.45+ (embedded mode)
  - PostgreSQL 16+ (server mode)

### 5.7. API Standards

* **OpenAPI Specification:** 3.1.0
* **REST API Versioning:** URL-based (`/api/v1/`)
* **Response Format:** JSON (RFC 8259)
* **Date/Time Format:** ISO 8601
* **Character Encoding:** UTF-8

---

## 6. ETL Pipeline Specification

### 6.1. Input Processing

#### Discovery Phase
- Recursively scan the dataset repository `src/` directory
- Identify all manufacturer directories (first level)
- Identify all model directories (second level)
- Collect all JSON files: `base.json`, year directories, and variant files

#### File Classification
- **Base Files**: `src/<make>/<model>/base.json`
- **Year Base Files**: `src/<make>/<model>/<year>/<vehicle_slug>.json`
- **Variant Files**: `src/<make>/<model>/<year>/<vehicle_slug>_<variant_slug>.json`

### 6.2. Merge Strategy

The ETL implements a **deterministic deep merge** with the following precedence (lowest to highest):

1. **Model Base** (`base.json`) - Shared attributes across all years
2. **Year Base** (`<vehicle_slug>.json`) - Specific year configuration
3. **Variant** (`<vehicle_slug>_<variant_slug>.json`) - Delta from year base

#### Merge Rules
- **Objects**: Deep merge by key (recursive)
- **Scalars** (string, number, boolean): Replace (higher precedence wins)
- **Arrays**: Complete replacement (no concatenation)
- **Null values**: Not allowed (use explicit empty states instead)
- **Unknown keys**: Validation failure

#### Output Cardinality
- Each year base file produces **one canonical vehicle**
- Each variant file produces **one additional canonical vehicle**
- Example: `model_3.json` + `model_3_long_range.json` = 2 canonical vehicles

### 6.3. Validation Pipeline

Each canonical vehicle must pass:

1. **JSON Schema Validation**: Against `schema.json` from dataset repository
2. **Required Fields Check**: All mandatory fields present
3. **Type Validation**: Correct data types for all fields
4. **Business Rules**:
   - At least one battery capacity (gross or net)
   - At least one charge port
   - At least one rated range entry
   - At least one source
   - Valid slug patterns (lowercase, alphanumeric + underscore)
   - Valid ISO codes (country, currency)
5. **Referential Integrity**: Variant files reference valid base vehicles

### 6.4. Output Formats

#### 6.4.1. Canonical JSON (`vehicles.json`)

**Format**: Single JSON file with array of all canonical vehicles

**Structure**:
```json
{
  "schema_version": "1.0.0",
  "generated_at": "2025-12-25T12:00:00Z",
  "vehicle_count": 1234,
  "vehicles": [
    { /* canonical vehicle 1 */ },
    { /* canonical vehicle 2 */ },
    ...
  ],
  "metadata": {
    "etl_version": "1.0.0",
    "dataset_commit": "abc123def",
    "processing_time_ms": 5432
  }
}
```

**Use Cases**: 
- Direct JSON consumption by applications
- Import into other systems
- Data analysis and exploration

#### 6.4.2. SQLite Database (`vehicles.db`)

**Schema Design**: Normalized relational structure

**Tables**:
- `vehicles` - Core vehicle information
- `battery_specs` - Battery specifications
- `charging_specs` - Charging capabilities
- `charge_ports` - Physical charge ports
- `motors` - Electric motor details
- `range_ratings` - Range by test cycle
- `sources` - Data sources
- `variants` - Variant metadata

**Indexes**:
- Primary keys on all tables
- Composite index on (make_slug, model_slug, year, trim_slug)
- Index on make_slug, model_slug separately
- Full-text search index on model names

**Use Cases**:
- Embedded applications
- Desktop applications
- Quick queries without server setup
- API server data source (embedded mode)

#### 6.4.3. PostgreSQL Schema (`vehicles.sql`)

**Contents**:
- Complete DDL (CREATE TABLE, CREATE INDEX)
- INSERT statements for all data
- Views for common queries
- Functions for search operations

**Features**:
- JSONB columns for complex nested data
- GiST indexes for JSONB queries
- Full-text search with tsvector
- Materialized views for aggregations

**Use Cases**:
- Production API deployments
- Advanced analytics
- Multi-tenant scenarios
- High-concurrency environments

#### 6.4.4. CSV Export (`vehicles.csv`)

**Format**: Flattened denormalized structure

**Columns**:
- Vehicle identification (make, model, year, trim, variant)
- Key specifications (battery, range, charging)
- Performance metrics
- Pricing information
- Source URLs (concatenated)

**Handling Complex Fields**:
- Arrays: Pipe-separated values (`value1|value2|value3`)
- Objects: Dot notation (`battery.pack_capacity_kwh_net`)
- Nested structures: Flattened to top level

**Use Cases**:
- Spreadsheet analysis (Excel, Google Sheets)
- Data science workflows (pandas, R)
- Business intelligence tools
- Legacy system integration

#### 6.4.5. XML Export (`vehicles.xml`)

**Format**: Hierarchical XML structure

**Root Element**: `<vehicles>`

**Vehicle Structure**:
```xml
<vehicle id="oed:tesla:model_3:2024:base">
  <make slug="tesla">Tesla</make>
  <model slug="model_3">Model 3</model>
  <year>2024</year>
  <battery>
    <pack_capacity_kwh_net>60.0</pack_capacity_kwh_net>
    <!-- ... -->
  </battery>
  <!-- ... -->
</vehicle>
```

**Features**:
- XML Schema (XSD) generation
- Namespace support
- XSLT transformation support

**Use Cases**:
- Enterprise system integration
- SOAP-based services
- Government/regulatory systems
- Legacy XML pipelines

### 6.5. Error Handling and Reporting

#### Validation Errors
- Collect all errors (don't fail fast)
- Categorize by severity: ERROR, WARNING, INFO
- Generate detailed error report with file paths and line numbers

#### Statistics Generation
- Total vehicles processed
- Variants generated
- Files scanned
- Validation failures
- Processing time per stage

#### Exit Codes
- `0`: Success - all vehicles valid
- `1`: Validation failures - some vehicles invalid
- `2`: Schema errors - malformed JSON
- `3`: File system errors - cannot read files

---

## 7. API Server Specification

### 7.1. Architecture

**Pattern**: Three-layer architecture
- **Presentation Layer**: HTTP handlers and routing
- **Service Layer**: Business logic and queries
- **Data Layer**: Database abstraction

### 7.2. Endpoints

#### GET `/api/v1/health`
Health check endpoint

**Response**:
```json
{
  "status": "healthy",
  "version": "1.0.0",
  "database": "connected",
  "vehicle_count": 1234
}
```

#### GET `/api/v1/vehicles`
List all vehicles with pagination and filtering

**Query Parameters**:
- `make`: Filter by manufacturer (slug)
- `model`: Filter by model (slug)
- `year`: Filter by year
- `vehicle_type`: Filter by type (suv, sedan, etc.)
- `min_range_km`: Minimum range
- `max_range_km`: Maximum range
- `page`: Page number (default: 1)
- `per_page`: Items per page (default: 20, max: 100)

**Response**:
```json
{
  "vehicles": [ /* vehicle summaries */ ],
  "pagination": {
    "page": 1,
    "per_page": 20,
    "total": 1234,
    "total_pages": 62
  }
}
```

#### GET `/api/v1/vehicles/{make}/{model}/{year}`
Get specific vehicle details

**Path Parameters**:
- `make`: Manufacturer slug
- `model`: Model slug
- `year`: Model year

**Response**: Full canonical vehicle object

#### GET `/api/v1/vehicles/{make}/{model}/{year}/variants`
List all variants for a specific vehicle

**Response**: Array of variant vehicles

#### GET `/api/v1/makes`
List all manufacturers

**Response**:
```json
{
  "makes": [
    { "slug": "tesla", "name": "Tesla", "vehicle_count": 42 },
    { "slug": "byd", "name": "BYD", "vehicle_count": 38 }
  ]
}
```

#### GET `/api/v1/makes/{make}/models`
List all models for a manufacturer

**Response**: Array of models with vehicle counts

#### GET `/api/v1/search`
Full-text search across vehicles

**Query Parameters**:
- `q`: Search query
- `page`, `per_page`: Pagination

**Response**: Ranked search results

### 7.3. Configuration

Configuration via environment variables:

- `DATABASE_URL`: SQLite or PostgreSQL connection string
- `PORT`: Server port (default: 3000)
- `HOST`: Bind address (default: 0.0.0.0)
- `LOG_LEVEL`: Logging level (debug, info, warn, error)
- `CORS_ORIGINS`: Allowed CORS origins
- `MAX_PAGE_SIZE`: Maximum items per page
- `ENABLE_COMPRESSION`: Enable gzip compression
- `ENABLE_OPENAPI`: Enable OpenAPI endpoint

### 7.4. Performance Characteristics

**Targets**:
- Cold start: < 100ms (with SQLite embedded)
- Response time (p50): < 10ms
- Response time (p99): < 50ms
- Throughput: > 10,000 req/s (single instance, cached)
- Memory footprint: < 100MB (with in-memory SQLite)

---

## 8. CI/CD Pipeline

### 8.1. Dataset Repository Pipeline

**Triggered by**: Push to `main` or release creation

**Workflow: ETL Artifacts Generation**

```yaml
name: Generate Data Artifacts

on:
  push:
    branches: [main]
  release:
    types: [created]

jobs:
  build-etl:
    - Checkout API repository
    - Install Rust toolchain
    - Build ev-etl in release mode
    
  generate-artifacts:
    - Run ev-etl against dataset source
    - Generate all output formats
    - Validate all artifacts
    - Calculate checksums
    
  publish-artifacts:
    - Upload to GitHub Release
    - Tag with semantic version
    - Generate release notes
```

**Artifacts Produced**:
- `vehicles.json` + `vehicles.json.sha256`
- `vehicles.db` + `vehicles.db.sha256`
- `vehicles.sql` + `vehicles.sql.sha256`
- `vehicles.csv` + `vehicles.csv.sha256`
- `vehicles.xml` + `vehicles.xml.sha256`
- `validation-report.txt`
- `statistics.json`

### 8.2. API Repository Pipeline

**Triggered by**: Push, PR, or release

**Workflow 1: Continuous Integration**

```yaml
name: CI

on: [push, pull_request]

jobs:
  test:
    - cargo fmt --check
    - cargo clippy -- -D warnings
    - cargo test --all-features
    - cargo test --doc
    
  build:
    - cargo build --release -p ev-etl
    - cargo build --release -p ev-server
```

**Workflow 2: Release**

```yaml
name: Release

on:
  push:
    branches: [main]

jobs:
  semantic-release:
    - Run semantic-release
    - Generate changelog
    - Create GitHub release
    
  build-binaries:
    - Cross-compile for multiple platforms
    - Upload binaries to release
    
  build-docker:
    - Build Docker image
    - Push to container registry
    - Tag with semantic version
```

**Release Artifacts**:
- Binaries: `ev-etl-linux-x64`, `ev-etl-windows-x64`, `ev-etl-macos-arm64`
- Binaries: `ev-server-linux-x64`, `ev-server-windows-x64`, `ev-server-macos-arm64`
- Docker image: `ghcr.io/open-ev-data/ev-server:latest`
- Docker image: `ghcr.io/open-ev-data/ev-server:v1.2.3`

---

## 9. Deployment Scenarios

### 9.1. Embedded Mode (SQLite)

**Use Case**: Single-server deployments, edge locations, development

**Setup**:
```bash
# Download artifacts
wget https://github.com/.../vehicles.db
wget https://github.com/.../ev-server-linux-x64

# Run server
DATABASE_URL=vehicles.db ./ev-server-linux-x64
```

**Characteristics**:
- Zero external dependencies
- < 100MB total footprint
- Single binary deployment
- Fast startup time

### 9.2. Container Deployment (Docker)

**Use Case**: Cloud deployments, Kubernetes, scalability

**Docker Compose**:
```yaml
version: '3.8'
services:
  api:
    image: ghcr.io/open-ev-data/ev-server:latest
    environment:
      - DATABASE_URL=vehicles.db
    ports:
      - "3000:3000"
    volumes:
      - ./vehicles.db:/app/vehicles.db:ro
```

**Kubernetes Deployment**:
- Deployment with multiple replicas
- ConfigMap for database file
- HorizontalPodAutoscaler
- Ingress for external access

### 9.3. PostgreSQL Mode

**Use Case**: High-concurrency production, multi-tenant

**Setup**:
```bash
# Import schema
psql -d openev -f vehicles.sql

# Run server
DATABASE_URL=postgresql://user:pass@localhost/openev ./ev-server
```

**Characteristics**:
- Advanced query capabilities
- Connection pooling
- Read replicas support
- Full ACID compliance

### 9.4. Serverless / Edge (Future)

**Platforms**: AWS Lambda, Cloudflare Workers, Vercel Edge

**Adaptations Required**:
- Separate WASM build target
- Remote database connection
- Cold start optimization
- Response streaming

---

## 10. Development Workflow

> **Important**: For detailed Rust coding standards, best practices, and implementation guidelines, see [RUST_GUIDELINES.md](./RUST_GUIDELINES.md).

### 10.1. Initial Setup

```bash
# Clone API repository
git clone https://github.com/open-ev-data/open-ev-data-api.git
cd open-ev-data-api

# Install Rust toolchain
rustup install stable
rustup default stable

# Build all crates
cargo build --all

# Run tests
cargo test --all
```

### 10.2. ETL Development Cycle

```bash
# Point to local dataset
export DATASET_PATH=../open-ev-data-dataset/src

# Build and run ETL
cargo run -p ev-etl -- \
  --input $DATASET_PATH \
  --output ./output \
  --formats json,sqlite,csv

# Validate output
cargo run -p ev-etl -- \
  --validate ./output/vehicles.json
```

### 10.3. Server Development Cycle

```bash
# Use test database
cargo run -p ev-server -- \
  --database ./output/vehicles.db \
  --port 3000

# Run with hot reload (cargo-watch)
cargo watch -x 'run -p ev-server'

# Run integration tests
cargo test -p ev-server --test integration
```

### 10.4. Release Process

1. **Dataset Release**: Triggers ETL artifact generation
2. **API Development**: Happens independently
3. **API Release**: semantic-release handles versioning
4. **Docker Build**: Automatic on API release
5. **Deployment**: Manual or automatic depending on environment

---

## 11. Future Enhancements

### Phase 2 - API Improvements
- GraphQL endpoint alongside REST
- WebSocket support for real-time updates
- Advanced search with Elasticsearch
- Caching layer with Redis
- Rate limiting and API keys

### Phase 3 - Edge Computing
- Cloudflare Workers deployment
- AWS Lambda@Edge support
- Global CDN distribution
- Sub-10ms response times worldwide

### Phase 4 - Analytics
- Usage analytics and telemetry
- Popular vehicle tracking
- Search query analysis
- Performance monitoring dashboard

### Phase 5 - Data Quality
- Automated data quality scoring
- Community contribution workflow
- Diff visualization for updates
- Historical data tracking (versioned snapshots)

---

## 12. Summary

The OpenEV Data API provides a comprehensive solution for transforming the layered dataset into multiple consumption formats:

**Core Strengths**:
1. **Build-Time Compilation**: Data validation happens once, not on every request
2. **Multiple Formats**: Single source, five output formats
3. **Type Safety**: Rust's type system prevents invalid states
4. **Performance**: Optimized for read-heavy workloads
5. **Portability**: Works embedded (SQLite) or client-server (PostgreSQL)
6. **Automation**: Fully automated CI/CD pipeline

**Architecture Benefits**:
- Clean separation of concerns (Hexagonal Architecture)
- Testable and maintainable codebase
- Independent deployability of components
- Multiple deployment strategies supported

**Integration Points**:
- Dataset updates trigger artifact regeneration
- API releases follow semantic versioning
- Multiple consumption patterns supported (files, API, embedded)
