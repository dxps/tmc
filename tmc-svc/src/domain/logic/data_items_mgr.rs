use std::sync::Arc;

use crate::{app::errors::AppError, domain::model::DataItemDef, repos::DataItemDefsRepo};

/// Data items manager contains the business logic that is beyond the structure of the data item definition itself.
#[derive(Clone)]
pub struct DataItemDefsMgr {
    repo: Arc<DataItemDefsRepo>,
}

impl DataItemDefsMgr {
    //
    pub fn new(repo: Arc<DataItemDefsRepo>) -> Self {
        Self { repo }
    }

    pub async fn get_all_attr_defs(&self) -> Result<Vec<DataItemDef>, AppError> {
        self.repo.get_all().await
    }
}
