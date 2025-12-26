use std::sync::Arc;

use axum::extract::{Query, State};
use axum::routing::get;
use axum::{Json, Router};

use crate::db::Database;
use crate::error::{ApiError, ProblemDetails};
use crate::models::{Pagination, SearchQuery, SearchResponse};

pub fn routes() -> Router<Arc<Database>> {
    Router::new().route("/search", get(search_vehicles))
}

#[utoipa::path(
    get,
    path = "/search",
    tag = "search",
    summary = "Search vehicles by keyword",
    description = "Full-text search across vehicle makes, models, and trims. Returns paginated results matching the query. The search is case-insensitive and matches partial words. Minimum query length is 2 characters.",
    params(
        ("q" = String, Query, description = "Search query (minimum 2 characters)"),
        ("page" = Option<usize>, Query, description = "Page number, starting from 1 (default: 1)"),
        ("per_page" = Option<usize>, Query, description = "Results per page, 1-100 (default: 20)")
    ),
    responses(
        (status = 200, description = "Search results with pagination", body = SearchResponse),
        (status = 400, description = "Invalid search query (empty or too short)", body = ProblemDetails),
        (status = 500, description = "Internal server error", body = ProblemDetails)
    )
)]
pub async fn search_vehicles(
    State(db): State<Arc<Database>>,
    Query(query): Query<SearchQuery>,
) -> Result<Json<SearchResponse>, ApiError> {
    if query.q.is_empty() {
        return Err(ApiError::BadRequest(
            "Search query cannot be empty".to_string(),
        ));
    }

    if query.q.len() < 2 {
        return Err(ApiError::BadRequest(
            "Search query must be at least 2 characters".to_string(),
        ));
    }

    let per_page = query.per_page.min(100).max(1);
    let page = query.page.max(1);

    let (vehicles, total) = db
        .search(&query.q, page, per_page)
        .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

    Ok(Json(SearchResponse {
        results: vehicles,
        pagination: Pagination::new(page, per_page, total),
    }))
}
