use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;

use crate::app::db::ping_db;
use crate::app::state::AppState;

pub async fn health_check(State(state): State<AppState>) -> impl IntoResponse {
    //
    match ping_db(&state.dbcp).await {
        true => Json(json!({ "database": "ok" })),
        false => Json(json!({ "database": "err" })),
    }
}
