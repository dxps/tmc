use axum::{extract::State, http::StatusCode, Json};
use serde_json::{json, Value};

use crate::app::state::AppState;

use super::respond_internal_server_error;

pub async fn get_attribute_definitions(State(state): State<AppState>) -> (StatusCode, Json<Value>) {
    //
    match state.attr_defs_mgr.get_all_attr_defs().await {
        Ok(res) => (StatusCode::OK, Json(json!(res))),
        Err(err) => respond_internal_server_error(err),
    }
}
