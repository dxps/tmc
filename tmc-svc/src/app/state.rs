use crate::{
    app::db::DbConnPool, domain::logic::AttributeDefinitionsMgr, repos::AttributeDefinitionsRepo,
};
use axum::extract::FromRef;
use std::sync::Arc;

/// The (global) state of the app.
#[derive(Clone, FromRef)]
pub struct AppState {
    pub dbcp: Arc<DbConnPool>,
    pub attr_defs_mgr: AttributeDefinitionsMgr,
}

impl AppState {
    //
    pub fn new(dbcp: DbConnPool) -> Self {
        //
        let dbcp = Arc::new(dbcp);
        let attr_defs_repo = Arc::new(AttributeDefinitionsRepo::new(dbcp.clone()));

        let attr_defs_mgr = AttributeDefinitionsMgr::new(attr_defs_repo.clone());

        Self {
            dbcp,
            attr_defs_mgr,
        }
    }
}
