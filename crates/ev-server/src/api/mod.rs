pub mod health;
pub mod makes;
pub mod search;
pub mod vehicles;

use std::sync::Arc;

use axum::Router;
use axum::http::header;
use tower_http::compression::CompressionLayer;
use tower_http::cors::{Any, CorsLayer};
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::config::Config;
use crate::db::Database;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "OpenEV Data API",
        version = env!("CARGO_PKG_VERSION"),
        description = "REST API for querying electric vehicle specifications",
        license(name = "AGPL-3.0", url = "https://www.gnu.org/licenses/agpl-3.0.html"),
        contact(name = "OpenEV Data", url = "https://open-ev-data.github.io/latest/")
    ),
    servers(
        (url = "/api/v1", description = "API v1")
    ),
    paths(
        health::health_check,
        vehicles::list_vehicles,
        vehicles::get_vehicle,
        vehicles::get_vehicle_variants,
        vehicles::get_vehicle_by_code,
        makes::list_makes,
        makes::list_models,
        search::search_vehicles,
    ),
    components(
        schemas(
            crate::error::ProblemDetails,
            crate::models::HealthResponse,
            crate::models::VehicleListResponse,
            crate::models::Pagination,
            crate::models::VehicleListQuery,
            crate::models::MakesListResponse,
            crate::models::ModelsListResponse,
            crate::models::SearchQuery,
            crate::models::SearchResponse,
            crate::db::VehicleSummary,
            crate::db::MakeSummary,
            crate::db::ModelSummary,
        )
    ),
    tags(
        (name = "health", description = "Server health and status monitoring"),
        (name = "vehicles", description = "Browse, filter, and retrieve electric vehicle specifications"),
        (name = "makes", description = "List vehicle manufacturers and their models"),
        (name = "search", description = "Full-text search across all vehicle data"),
    )
)]
pub struct ApiDoc;

pub fn create_router(db: Arc<Database>, config: &Config) -> Router {
    let api_routes = Router::new()
        .merge(health::routes())
        .merge(vehicles::routes())
        .merge(makes::routes())
        .merge(search::routes())
        .with_state(db);

    let mut app = Router::new().nest("/api/v1", api_routes);

    if config.enable_openapi {
        app = app.merge(SwaggerUi::new("/docs").url("/openapi.json", ApiDoc::openapi()));
    }

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]);

    let timeout = TimeoutLayer::with_status_code(
        axum::http::StatusCode::REQUEST_TIMEOUT,
        std::time::Duration::from_secs(30),
    );
    let mut app = app
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .layer(timeout);

    if config.enable_compression {
        app = app.layer(CompressionLayer::new());
    }

    app
}
