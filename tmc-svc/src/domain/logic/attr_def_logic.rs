use std::sync::Arc;

use crate::{
    app::errors::AppError, domain::model::AttributeDefinition, repos::AttributeDefinitionsRepo,
};

#[derive(Clone)]
pub struct AttributeDefinitionsMgr {
    repo: Arc<AttributeDefinitionsRepo>,
}

impl AttributeDefinitionsMgr {
    //
    pub fn new(repo: Arc<AttributeDefinitionsRepo>) -> Self {
        Self { repo }
    }

    pub async fn get_all_attr_defs(&self) -> Result<Vec<AttributeDefinition>, AppError> {
        self.repo.get_all().await
    }
}
