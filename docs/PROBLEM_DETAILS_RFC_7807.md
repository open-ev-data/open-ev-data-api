# RFC 7807 Problem Details

This API uses [RFC 7807 Problem Details](https://datatracker.ietf.org/doc/html/rfc7807) for standardized error responses.

---

## Response Format

All error responses return a JSON object with these fields:

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `type` | string | Yes | URL to the documentation for this error type |
| `title` | string | Yes | Short, human-readable summary |
| `status` | integer | Yes | HTTP status code |
| `detail` | string | Yes | Specific explanation of this occurrence |
| `instance` | string | No | URI of the specific request that caused the error |

---

## Example Response

```json
{
  "type": "https://github.com/open-ev-data/open-ev-data-api/blob/main/docs/API_ERRORS.md#errorsnot-found",
  "title": "Resource Not Found",
  "status": 404,
  "detail": "Vehicle not found with code: tesla:model_x:2099:model_x",
  "instance": "/api/v1/vehicles/code/tesla:model_x:2099:model_x"
}
```

---

## Implementation

### Adding New Error Types

1. Define the error variant in `error.rs`:

```rust
pub enum ApiError {
    NotFound(String),
    BadRequest(String),
    // Add new variants here
}
```

2. Map it in `to_problem_details()`:

```rust
Self::NewError(detail) => (
    StatusCode::UNPROCESSABLE_ENTITY,
    ProblemDetails::new(
        "https://github.com/open-ev-data/open-ev-data-api/blob/main/docs/API_ERRORS.md#errorsnew-error",
        "New Error Title",
        StatusCode::UNPROCESSABLE_ENTITY,
        detail.clone(),
    ),
),
```

3. Document in [API_ERRORS.md](./API_ERRORS.md) with matching anchor ID.

---

## References

- [RFC 7807 - Problem Details for HTTP APIs](https://datatracker.ietf.org/doc/html/rfc7807)
- [IETF HTTP Problem Types Registry](https://www.iana.org/assignments/http-problem-types)
- [API Error Catalog](./API_ERRORS.md)
