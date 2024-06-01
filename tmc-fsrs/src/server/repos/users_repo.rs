use sqlx::{postgres::PgRow, FromRow, PgPool, Row};
use std::sync::Arc;

use crate::server::{
    domain::{UserAccount, UserEntry},
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
            "SELECT id, email, username, password, salt, bio, image, anonymous FROM users_accounts 
             WHERE email = $1",
        )
        .bind(email)
        .fetch_one(self.dbcp.as_ref())
        .await
        .map_err(|err| AppError::from((err, usecase)))
    }

    pub async fn get_by_id(id: i64, pool: &PgPool) -> Option<UserAccount> {
        //
        let mut user_account = sqlx::query_as::<_, UserAccount>(
            "SELECT id, email, username, bio, image, anyonymous FROM users_accounts WHERE id = $1",
        )
        .bind(id)
        .fetch_one(pool)
        .await
        .ok()?;

        let mut permissions = sqlx::query("SELECT permission FROM users_permissions WHERE user_id = $1;")
            .map(|r: PgRow| r.get("permission"))
            .fetch_all(pool)
            .await
            .ok()?;

        user_account.permissions.append(&mut permissions);
        Some(user_account)
    }

    pub async fn save(&self, email: String, username: String, pwd: String, salt: String) -> Result<i64, AppError> {
        //
        match sqlx::query(
            "INSERT INTO users_accounts (email, username, password, salt) 
             VALUES ($1, $2, $3, $4) RETURNING id",
        )
        .bind(email)
        .bind(username)
        .bind(pwd)
        .bind(salt)
        .fetch_one(self.dbcp.as_ref())
        .await
        {
            Ok(row) => Ok(row.get("id")),
            Err(err) => Err(AppError::from((err, AppUseCase::UserRegistration))),
        }
    }
}

// -----------------------------------
//    sqlx::FromRow implementations
// -----------------------------------

impl FromRow<'_, PgRow> for UserAccount {
    //
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.get("id"),
            email: row.get("email"),
            username: row.get("username"),
            bio: row.get("bio"),
            image: row.get("image"),
            anonymous: row.get("anonymous"),
            permissions: Vec::new(),
        })
    }
}

impl FromRow<'_, PgRow> for UserEntry {
    //
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            user: UserAccount {
                id: row.try_get("id").unwrap_or_default(),
                email: row.get("email"),
                username: row.get("username"),
                bio: row.get("bio"),
                image: row.try_get("image").unwrap_or_default(),
                anonymous: row.get("anonymous"),
                permissions: Vec::new(),
            },
            password: row.get("password"),
            salt: row.get("salt"),
        })
    }
}

impl From<(sqlx::Error, AppUseCase)> for AppError {
    //
    fn from(ctx: (sqlx::Error, AppUseCase)) -> Self {
        log::debug!("from((sqlx::Error, AppUseCase)): ctx={:?}", ctx);
        let err = ctx.0;
        // Start with the use case as the context, and then cover the possible errors within.
        match ctx.1 {
            AppUseCase::UserRegistration => match &err.into_database_error() {
                Some(e) => match e.code() {
                    Some(code) => match code.as_ref() {
                        "23505" => AppError::AlreadyExists("email".into()),
                        _ => AppError::InternalErr,
                    },
                    None => AppError::InternalErr,
                },
                None => AppError::InternalErr,
            },

            AppUseCase::UserLogin => match &err {
                sqlx::Error::RowNotFound => AppError::Unauthorized("wrong credentials".into()),
                _ => AppError::InternalErr,
            },

            AppUseCase::GetUserProfile => match &err {
                sqlx::Error::RowNotFound => AppError::NotFound("profile".into()),
                _ => AppError::InternalErr,
            },
        }
    }
}

impl From<sqlx::Error> for AppError {
    //
    fn from(err: sqlx::Error) -> Self {
        let mut app_err = AppError::Ignorable;
        log::debug!("from(sqlx:Error): err={:?}", err);
        if err.as_database_error().is_some() {
            // TODO: For now, any db error is classified as internal error.
            app_err = AppError::InternalErr
        }
        app_err
    }
}
