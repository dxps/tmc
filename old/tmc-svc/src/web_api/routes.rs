use axum::routing::get;
use axum::Router;
use tower_http::{
    cors::{AllowHeaders, Any, CorsLayer},
    trace::TraceLayer,
};

use crate::app::state::AppState;

use super::{get_data_item_defs, health_check};

/// Init the Router and its routes
pub fn init_router(state: AppState) -> Router {
    //
    let cors_layer = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(AllowHeaders::any());
    let log_layer = TraceLayer::new_for_http();
    let tracing_layer = TraceLayer::new_for_http();
    Router::new()
        .route("/api/healthcheck", get(health_check))
        .route("/api/data_item_defs", get(get_data_item_defs))
        .layer(cors_layer)
        .layer(log_layer)
        .layer(tracing_layer)
        .with_state(state)
}
