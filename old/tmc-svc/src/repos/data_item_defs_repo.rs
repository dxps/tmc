use std::sync::Arc;

use sqlx::{postgres::PgRow, Row};

use crate::{
    app::{db::DbConnPool, errors::AppError},
    domain::model::DataItemDef,
};

/// The repository of data items definitions.
#[derive(Clone)]
pub struct DataItemDefsRepo {
    dbcp: Arc<DbConnPool>,
}

impl DataItemDefsRepo {
    //
    pub fn new(dbcp: Arc<DbConnPool>) -> Self {
        Self { dbcp }
    }

    pub async fn get_all(&self) -> Result<Vec<DataItemDef>, AppError> {
        //
        let conn = self.dbcp.as_ref();
        let res = sqlx::query_as::<_, DataItemDef>(
            "SELECT id, name, description, data_type FROM attribute_definitions",
        )
        .fetch_all(conn)
        .await;
        Ok(res?)
    }
}

impl sqlx::FromRow<'_, PgRow> for DataItemDef {
    //
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            data_type: row.get::<'_, String, _>("data_type").into(),
        })
    }
}
