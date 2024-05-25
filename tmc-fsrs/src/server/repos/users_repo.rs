use sqlx::{postgres::PgRow, FromRow, PgPool, Row};
use std::sync::Arc;

use crate::server::{
    model::{User, UserEntry},
    AppError, AppUseCase,
};

pub struct UsersRepo {
    dbcp: Arc<PgPool>,
}

impl UsersRepo {
    //
    pub fn new(dbcp: Arc<PgPool>) -> Self {
        Self { dbcp }
    }

    pub async fn get_by_email(&self, email: &String, usecase: AppUseCase) -> Result<UserEntry, AppError> {
        //
        sqlx::query_as::<_, UserEntry>(
            "SELECT id, email, username, password, salt, bio, image FROM users_accounts 
             WHERE email = $1",
        )
        .bind(email)
        .fetch_one(self.dbcp.as_ref())
        .await
        .map_err(|err| AppError::from((err, usecase)))
    }
}

// -----------------------------------
//    sqlx::FromRow implementations
// -----------------------------------

impl FromRow<'_, PgRow> for User {
    //
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.get("id"),
            email: row.get("email"),
            username: row.get("username"),
            bio: row.get("bio"),
            image: row.get("image"),
        })
    }
}

impl FromRow<'_, PgRow> for UserEntry {
    //
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            user: User {
                id: row.try_get("id").unwrap_or_default(),
                email: row.get("email"),
                username: row.get("username"),
                bio: row.get("bio"),
                image: row.try_get("image").unwrap_or_default(),
            },
            password: row.get("password"),
            salt: row.get("salt"),
        })
    }
}
