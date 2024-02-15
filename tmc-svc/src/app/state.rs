use crate::app::db::DbConnPool;
use axum::extract::FromRef;
use std::sync::Arc;

/// The (global) state of the app.
#[derive(Clone, FromRef)]
pub struct AppState {
    pub dbcp: Arc<DbConnPool>,
}

impl AppState {
    //
    pub fn new(dbcp: DbConnPool) -> Self {
        let dbcp = Arc::new(dbcp);

        Self { dbcp }
    }
}
