# Docker Configuration

This directory contains Docker-related files for the OpenEV Data API project.

## Files Overview

| File | Purpose | Use Case |
|------|---------|----------|
| `Dockerfile` | Production build for ev-server | CI/CD pipelines, requires pre-built binary |
| `Dockerfile.etl` | Production build for ev-etl | CI/CD pipelines, requires pre-built binary |
| `Dockerfile.dev` | Development build for ev-server | Local development, compiles inside container |
| `Dockerfile.etl.dev` | Development build for ev-etl | Local development, compiles inside container |
| `docker-compose.yml` | Multi-service orchestration | Local development and testing |

## Usage

### Local Development

Use the development Dockerfiles via docker-compose:

```bash
docker-compose up
```

To run the ETL service:

```bash
docker-compose --profile etl up etl
```

### Production Deployment

Build binaries first, then create Docker images:

```bash
cargo build --release
docker build -t ghcr.io/open-ev-data/ev-server:latest -f docker/Dockerfile .
docker build -t ghcr.io/open-ev-data/ev-etl:latest -f docker/Dockerfile.etl .
```

## CI/CD Integration

The production Dockerfiles are optimized for CI/CD pipelines where:

1. Binaries are compiled separately with full test coverage
2. Docker images are built using pre-compiled binaries
3. Build time is minimized by avoiding duplicate compilation
4. Images are smaller as they don't include build dependencies

See `.github/workflows/release.yml` for the complete CI/CD pipeline.
