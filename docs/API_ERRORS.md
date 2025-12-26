# API Errors

All errors follow [RFC 7807 Problem Details](./PROBLEM_DETAILS_RFC_7807.md).

---

## Error Types

### `/errors/not-found` {#errorsnot-found}

**Status:** 404 Not Found

**When:** The requested resource does not exist.

**Common causes:**
- Vehicle with specified make/model/year does not exist
- Manufacturer slug is invalid or has no models

**Resolution:** Verify the resource identifier. Use `/api/v1/makes` to list valid manufacturers or `/api/v1/vehicles` to browse available vehicles.

**Example:**
```json
{
  "type": "https://github.com/open-ev-data/open-ev-data-api/blob/main/docs/API_ERRORS.md#errorsnot-found",
  "title": "Resource Not Found",
  "status": 404,
  "detail": "Vehicle not found: invalid/model/2024",
  "instance": "/api/v1/vehicles/invalid/model/2024"
}
```

---

### `/errors/bad-request` {#errorsbad-request}

**Status:** 400 Bad Request

**When:** The request contains invalid parameters.

**Common causes:**
- Search query is empty or too short (minimum 2 characters)
- Invalid query parameter values or types

**Resolution:** Check the `detail` field for specific validation errors. Review the API documentation for parameter requirements.

**Example:**
```json
{
  "type": "https://github.com/open-ev-data/open-ev-data-api/blob/main/docs/API_ERRORS.md#errorsbad-request",
  "title": "Invalid Request",
  "status": 400,
  "detail": "Search query must be at least 2 characters",
  "instance": "/api/v1/search"
}
```

---

### `/errors/internal-error` {#errorsinternal-error}

**Status:** 500 Internal Server Error

**When:** An unexpected error occurred on the server.

**Common causes:**
- Database connection failure
- Unexpected server-side exception

**Resolution:** Retry the request. If the problem persists, report the issue with the `instance` URI.

**Example:**
```json
{
  "type": "https://github.com/open-ev-data/open-ev-data-api/blob/main/docs/API_ERRORS.md#errorsinternal-error",
  "title": "Internal Server Error",
  "status": 500,
  "detail": "Database query failed",
  "instance": "/api/v1/vehicles"
}
```

---

### `/errors/service-unavailable` {#errorsservice-unavailable}

**Status:** 503 Service Unavailable

**When:** The service is temporarily unavailable.

**Common causes:**
- Database is offline or unreachable
- Server is overloaded

**Resolution:** Wait and retry after a few seconds. Check service status if issue persists.

**Example:**
```json
{
  "type": "https://github.com/open-ev-data/open-ev-data-api/blob/main/docs/API_ERRORS.md#errorsservice-unavailable",
  "title": "Service Unavailable",
  "status": 503,
  "detail": "Database connection unavailable",
  "instance": "/api/v1/health"
}
```

---

## Endpoints and Possible Errors

| Endpoint | Possible Errors |
|----------|-----------------|
| `GET /health` | 500, 503 |
| `GET /makes` | 500 |
| `GET /makes/{make}/models` | 404, 500 |
| `GET /vehicles` | 400, 500 |
| `GET /vehicles/code/{unique_code}` | 404, 500 |
| `GET /vehicles/{make}/{model}/{year}` | 404, 500 |
| `GET /vehicles/{make}/{model}/{year}/variants` | 404, 500 |
| `GET /search` | 400, 500 |
