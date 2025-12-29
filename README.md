<div align="center">
  <img src="https://raw.githubusercontent.com/open-ev-data/.github/bd1f96d4c236e7c1dc43a33535fc9f065f9b152e/assets/open-ev-data-logo.svg" alt="OpenEV Data Logo" width="200" />
</div>

# OpenEV Data API

> A high-performance REST API for querying electric vehicle specifications, built with Rust.

[![License: AGPL-3.0](https://img.shields.io/badge/License-AGPL--3.0-blue.svg)](LICENCE)
[![Version](https://img.shields.io/github/v/release/open-ev-data/open-ev-data-api?label=version)](https://github.com/open-ev-data/open-ev-data-api/releases)
[![Rust](https://img.shields.io/badge/rust-1.92.0+-orange.svg)](https://www.rust-lang.org)
[![Coverage](https://img.shields.io/badge/coverage-94.14%25-brightgreen)](https://github.com/open-ev-data/open-ev-data-api/releases/latest/download/coverage-report.tar.gz)

## Overview

This repository provides a **production-ready REST API** for querying electric vehicle data from the OpenEV Data Dataset. Built with Rust for maximum performance, type safety and reliability.

## Documentation

### Getting Started
- **[Get Started](docs/GET_STARTED.md)** — Setup, build, run, and test the system

### Technical Documentation
- **[Architecture](docs/ARCHITECTURE.md)** — Hexagonal architecture and design decisions
- **[Rust Guidelines](docs/RUST_GUIDELINES.md)** — Coding standards and best practices

### Crate Documentation
- **[ev-core](docs/CRATE_EV_CORE.md)** — Domain types and validation
- **[ev-etl](docs/CRATE_EV_ETL.md)** — ETL pipeline for data processing
- **[ev-server](docs/CRATE_EV_SERVER.md)** — REST API server with OpenAPI

### Project Governance
- **[Contributing Guide](https://github.com/open-ev-data/.github/blob/main/CONTRIBUTING.md)** — How to contribute
- **[Code of Conduct](https://github.com/open-ev-data/.github/blob/main/CODE_OF_CONDUCT.md)** — Community standards

## Features

- **High Performance**: Built with Rust and Axum for maximum throughput
- **Type Safety**: Strong typing with compile-time guarantees
- **Multiple Formats**: JSON, SQLite, PostgreSQL, CSV, XML output support
- **OpenAPI/Swagger**: Interactive API documentation
- **Pagination**: Efficient handling of large datasets
- **Full-text Search**: Search across manufacturers, models, and trims

## API Endpoints

| Endpoint | Description |
|----------|-------------|
| `GET /api/v1/health` | Health check |
| `GET /api/v1/vehicles/list` | List vehicles with filters and pagination |
| `GET /api/v1/vehicles/code/{code}` | Get vehicle by unique code |
| `GET /api/v1/vehicles/search?q=query` | Full-text search |
| `GET /api/v1/makes/list` | List manufacturers with model names |

## Use Cases

- **Applications**: EV comparison tools, range calculators, charging time estimators
- **Integrations**: Backend data source for automotive applications
- **Analytics**: EV market analysis, charging infrastructure planning
- **Research**: Academic studies on EV technology and adoption

## License

This project is licensed under the **GNU Affero General Public License v3.0 (AGPL-3.0)**.

See [LICENCE](LICENCE) for full terms.

## Acknowledgments

This project is part of the OpenEV Data initiative, providing open, structured, and verifiable EV data. Special thanks to all contributors who help maintain and improve this API.
