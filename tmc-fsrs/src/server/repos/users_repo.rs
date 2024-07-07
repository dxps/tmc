use sqlx::{postgres::PgRow, FromRow, PgPool, Row};
use std::sync::Arc;

use crate::server::{
    create_id,
    domain::{UserAccount, UserEntry},
    AppError, AppResult, AppUseCase, UserPasswordSalt,
};

pub struct UsersRepo {
    dbcp: Arc<PgPool>,
}

impl UsersRepo {
    //
    pub fn new(dbcp: Arc<PgPool>) -> Self {
        Self { dbcp }
    }

    pub async fn get_by_email(&self, email: &String, usecase: AppUseCase) -> AppResult<UserEntry> {
        //
        sqlx::query_as::<_, UserEntry>(
            "SELECT id, email, username, password, salt, bio, image, is_anonymous FROM user_accounts 
             WHERE email = $1",
        )
        .bind(email)
        .fetch_one(self.dbcp.as_ref())
        .await
        .map_err(|err| AppError::from((err, usecase)))
    }

    pub async fn get_by_id(id: String, pool: &PgPool) -> Option<UserAccount> {
        //
        let mut user_account = sqlx::query_as::<_, UserAccount>(
            "SELECT id, email, username, bio, image, is_anyonymous FROM user_accounts WHERE id = $1",
        )
        .bind(id)
        .fetch_one(pool)
        .await
        .ok()?;

        let mut permissions = sqlx::query("SELECT permission FROM user_permissions WHERE user_id = $1;")
            .map(|r: PgRow| r.get("permission"))
            .fetch_all(pool)
            .await
            .ok()?;

        user_account.permissions.append(&mut permissions);
        Some(user_account)
    }

    pub async fn get_password_by_id(&self, user_id: &String) -> AppResult<UserPasswordSalt> {
        //
        sqlx::query_as::<_, UserPasswordSalt>("SELECT password, salt FROM user_accounts WHERE id = $1")
            .bind(user_id)
            .fetch_one(self.dbcp.as_ref())
            .await
            .map_err(|err| AppError::from(err))
    }

    pub async fn update_password(&self, user_id: String, pwd: String) -> AppResult<()> {
        //
        match sqlx::query("UPDATE user_accounts SET password = $1 WHERE id = $2")
            .bind(pwd)
            .bind(user_id)
            .execute(self.dbcp.as_ref())
            .await
            .map_err(|err| AppError::from(err))
        {
            Ok(_) => Ok(()),
            Err(err) => Err(AppError::from(err)),
        }
    }

    pub async fn save(&self, email: String, username: String, pwd: String, salt: String) -> AppResult<String> {
        //
        let id = create_id();
        match sqlx::query(
            "INSERT INTO user_accounts (id, email, username, password, salt) 
             VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(&id)
        .bind(email)
        .bind(username)
        .bind(pwd)
        .bind(salt)
        .execute(self.dbcp.as_ref())
        .await
        {
            Ok(_) => Ok(id),
            Err(err) => Err(AppError::from((err, AppUseCase::UserRegistration))),
        }
    }

    pub async fn update(&self, ua: UserAccount) -> AppResult<()> {
        //
        match sqlx::query("UPDATE user_accounts SET username=$1, email=$2, bio=$3 WHERE id = $4")
            .bind(ua.username)
            .bind(ua.email)
            .bind(ua.bio)
            .bind(ua.id)
            .execute(self.dbcp.as_ref())
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(AppError::from(err)),
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
            is_anonymous: row.get("is_anonymous"),
            permissions: Vec::new(),
            attributes: Vec::new(),
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
                is_anonymous: row.get("is_anonymous"),
                permissions: Vec::new(),
                attributes: Vec::new(),
            },
            password: row.get("password"),
            salt: row.get("salt"),
        })
    }
}

impl FromRow<'_, PgRow> for UserPasswordSalt {
    //
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            password: row.get("password"),
            salt: row.get("salt"),
        })
    }
}

impl From<(sqlx::Error, AppUseCase)> for AppError {
    //
    fn from(ctx: (sqlx::Error, AppUseCase)) -> Self {
        //
        let err = &ctx.0;
        // Start with the use case as the context, and then cover the possible errors within.
        match ctx.1 {
            AppUseCase::UserRegistration => match &err.as_database_error() {
                Some(e) => match e.code() {
                    Some(code) => match code.as_ref() {
                        "23505" => AppError::AlreadyExists("email".into()),
                        _ => log_and_return_internal_err(ctx),
                    },
                    None => log_and_return_internal_err(ctx),
                },
                None => log_and_return_internal_err(ctx),
            },

            AppUseCase::UserLogin => match &err {
                sqlx::Error::RowNotFound => AppError::Unauthorized("wrong credentials".into()),
                _ => log_and_return_internal_err(ctx),
            },
        }
    }
}

fn log_and_return_internal_err(ctx: (sqlx::Error, AppUseCase)) -> AppError {
    log::debug!("InternalErr due to err={:?} on usecase:{:?}.", ctx.0, ctx.1);
    AppError::InternalErr
}

impl From<sqlx::Error> for AppError {
    //
    fn from(err: sqlx::Error) -> Self {
        //
        let mut app_err = AppError::Ignorable;
        log::debug!("from(sqlx:Error): err={:?}", err);
        if err.as_database_error().is_some() {
            // TODO: For now, any db error is considered as internal error.
            app_err = AppError::InternalErr
        }
        app_err
    }
}
