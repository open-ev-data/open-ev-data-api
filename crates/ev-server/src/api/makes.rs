use std::sync::Arc;

use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};

use crate::db::Database;
use crate::error::{ApiError, ProblemDetails};
use crate::models::MakesListResponse;

pub fn routes() -> Router<Arc<Database>> {
    Router::new().route("/makes/list", get(list_makes))
}

#[utoipa::path(
    get,
    path = "/makes/list",
    tag = "makes",
    summary = "List all vehicle manufacturers",
    description = "Returns a list of all vehicle manufacturers (makes) available in the database. Each manufacturer includes its slug, display name, vehicle count, and an array of unique model names.",
    responses(
        (status = 200, description = "List of all manufacturers with vehicle counts and model names", body = MakesListResponse),
        (status = 500, description = "Internal server error", body = ProblemDetails)
    )
)]
pub async fn list_makes(
    State(db): State<Arc<Database>>,
) -> Result<Json<MakesListResponse>, ApiError> {
    let makes = db
        .list_makes()
        .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

    Ok(Json(MakesListResponse { makes }))
}
