use std::sync::Arc;

use axum::extract::{Path, State};
use axum::routing::get;
use axum::{Json, Router};

use crate::db::Database;
use crate::error::{ApiError, ProblemDetails};
use crate::models::{MakesListResponse, ModelsListResponse};

pub fn routes() -> Router<Arc<Database>> {
    Router::new()
        .route("/makes", get(list_makes))
        .route("/makes/{make}/models", get(list_models))
}

#[utoipa::path(
    get,
    path = "/makes",
    tag = "makes",
    summary = "List all vehicle manufacturers",
    description = "Returns a list of all vehicle manufacturers (makes) available in the database. Each manufacturer includes its slug (URL-safe identifier), display name, and total vehicle count.",
    responses(
        (status = 200, description = "List of all manufacturers with vehicle counts", body = MakesListResponse),
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

#[utoipa::path(
    get,
    path = "/makes/{make}/models",
    tag = "makes",
    summary = "List models for a manufacturer",
    description = "Returns all models for a specific manufacturer. Each model includes available years and variant count. Use the manufacturer slug from the /makes endpoint.",
    params(
        ("make" = String, Path, description = "Manufacturer slug (e.g., 'tesla', 'byd', 'volkswagen')")
    ),
    responses(
        (status = 200, description = "List of models for the manufacturer", body = ModelsListResponse),
        (status = 404, description = "Manufacturer not found or has no models", body = ProblemDetails),
        (status = 500, description = "Internal server error", body = ProblemDetails)
    )
)]
pub async fn list_models(
    State(db): State<Arc<Database>>,
    Path(make): Path<String>,
) -> Result<Json<ModelsListResponse>, ApiError> {
    let models = db
        .list_models(&make)
        .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

    if models.is_empty() {
        return Err(ApiError::NotFound(format!(
            "No models found for manufacturer: {}",
            make
        )));
    }

    Ok(Json(ModelsListResponse { models }))
}
