use crate::{app::db::DbConnPool, domain::logic::DataItemDefsMgr, repos::DataItemDefsRepo};
use axum::extract::FromRef;
use std::sync::Arc;

/// The (global) state of the app.
#[derive(Clone, FromRef)]
pub struct AppState {
    pub dbcp: Arc<DbConnPool>,
    pub data_item_defs_mgr: DataItemDefsMgr,
}

impl AppState {
    //
    pub fn new(dbcp: DbConnPool) -> Self {
        //
        let dbcp = Arc::new(dbcp);

        let attr_defs_repo = Arc::new(DataItemDefsRepo::new(dbcp.clone()));
        let data_item_defs_mgr = DataItemDefsMgr::new(attr_defs_repo.clone());

        Self {
            dbcp,
            data_item_defs_mgr,
        }
    }
}
