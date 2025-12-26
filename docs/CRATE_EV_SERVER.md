# ev-server

REST API server for OpenEV Data.

## Overview

This crate provides a high-performance HTTP REST API server for querying electric vehicle data using the Axum web framework.

## Usage

```bash
DATABASE_URL=vehicles.db ev-server
```

## Environment Variables

- `DATABASE_URL`: Path to SQLite database (default: vehicles.db)
- `PORT`: Server port (default: 3000)
- `HOST`: Bind address (default: 0.0.0.0)
- `LOG_LEVEL`: Logging level - debug, info, warn, error (default: info)
- `CORS_ORIGINS`: Comma-separated allowed origins (default: *)
- `MAX_PAGE_SIZE`: Maximum items per page (default: 100)
- `ENABLE_COMPRESSION`: Enable gzip compression (default: true)
- `ENABLE_OPENAPI`: Enable Swagger UI (default: true)

## Endpoints

- `GET /api/v1/health` - Health check
- `GET /api/v1/vehicles` - List vehicles with filters
- `GET /api/v1/vehicles/{make}/{model}/{year}` - Get vehicle details
- `GET /api/v1/vehicles/{make}/{model}/{year}/variants` - List variants
- `GET /api/v1/makes` - List manufacturers
- `GET /api/v1/makes/{make}/models` - List models
- `GET /api/v1/search?q=query` - Full-text search

## OpenAPI Documentation

When enabled, Swagger UI is available at `/swagger-ui`.
