# Getting Started with OpenEV Data API

This guide covers the API-specific setup. For general environment setup (cloning repositories and opening the workspace), see the [Governance Getting Started Guide](https://github.com/open-ev-data/.github/blob/main/GET_STARTED.md).

## Prerequisites

Ensure you have completed the [Governance Getting Started Guide](https://github.com/open-ev-data/.github/blob/main/GET_STARTED.md) and have:

- All repositories cloned in the `open-ev-data/` directory
- The multi-repository workspace open in VS Code or Cursor
- Rust toolchain installed (1.92.0+)

Verify Rust installation:

```bash
rustc --version
cargo --version
```

---

## Build

From the `open-ev-data-api` directory:

```bash
cargo build --all
```

For production builds:

```bash
cargo build --release
```

Run tests to verify installation:

```bash
cargo test --all
```

---

## Run the ETL Pipeline

The ETL reads layered JSON from the Dataset repository and generates output artifacts.

```bash
cargo run -p ev-etl -- \
  --input ../open-ev-data-dataset/src \
  --output ./output \
  --formats json,sqlite
```

Available formats: `json`, `sqlite`, `postgresql`, `csv`, `xml`

Generated files:

| File | Description |
|------|-------------|
| `output/vehicles.json` | Canonical JSON with all vehicles |
| `output/vehicles.db` | SQLite database |
| `output/statistics.json` | Processing statistics |

Verify output:

```bash
cat output/vehicles.json | jq '.vehicle_count'
sqlite3 output/vehicles.db "SELECT COUNT(*) FROM vehicles;"
```

---

## Run the API Server

```bash
DATABASE_URL=./output/vehicles.db cargo run -p ev-server
```

Or with hot-reload:

```bash
DATABASE_URL=./output/vehicles.db cargo watch -x "run -p ev-server"
```

The server starts at `http://localhost:3000`.

Test endpoints:

```bash
curl http://localhost:3000/api/v1/health
curl http://localhost:3000/api/v1/makes/list
curl http://localhost:3000/api/v1/vehicles/list
curl http://localhost:3000/api/v1/vehicles/code/tesla:model_3:2024:model_3
curl "http://localhost:3000/api/v1/vehicles/search?q=dolphin"
```

Swagger UI is available at: `http://localhost:3000/docs`

---

## Development Workflow

1. Edit vehicle JSON files in `open-ev-data-dataset/src`
2. Run ETL to regenerate artifacts
3. Restart the API server
4. Test changes via API endpoints

For hot-reload during development:

```bash
cargo install cargo-watch
cargo watch -x 'run -p ev-server -- --database ./output/vehicles.db'
```

---

## Docker

### Development Build

For local development (builds inside the container):

```bash
docker build -t ev-server -f docker/Dockerfile.dev .
docker run -p 3000:3000 -v $(pwd)/output/vehicles.db:/app/vehicles.db:ro ev-server
```

Or use Docker Compose:

```bash
docker-compose -f docker/docker-compose.yml up
```

### Production Build

For CI/CD and production deployments (uses pre-compiled binaries):

```bash
cargo build --release
docker build -t ev-server -f docker/Dockerfile .
docker run -p 3000:3000 -v $(pwd)/output/vehicles.db:/app/vehicles.db:ro ev-server
```

The production Dockerfile expects binaries to be pre-built in `target/release/`, which significantly reduces build time and image size.

---

## Command Reference

| Command | Description |
|---------|-------------|
| `cargo build --all` | Build all crates |
| `cargo test --all` | Run all tests |
| `cargo fmt --all` | Format code |
| `cargo clippy --all` | Run linter |
| `cargo run -p ev-etl -- --help` | ETL options |
| `cargo run -p ev-server -- --help` | Server options |

---

## Troubleshooting

| Issue | Solution |
|-------|----------|
| `cargo: command not found` | Install Rust via [rustup.rs](https://rustup.rs/) |
| Database not found | Run ETL first to generate `output/vehicles.db` |
| Port already in use | Use `PORT=8080 cargo run -p ev-server` |

---

## Further Reading

- [Architecture](./ARCHITECTURE.md)
- [Rust Guidelines](./RUST_GUIDELINES.md)
- [ev-core Crate](./CRATE_EV_CORE.md)
- [ev-etl Crate](./CRATE_EV_ETL.md)
- [ev-server Crate](./CRATE_EV_SERVER.md)
