use async_trait::async_trait;
use axum::response::{IntoResponse, Response};
use axum_session_auth::*;
use axum_session_sqlx::SessionPgPool;
use sqlx::PgPool;
use std::sync::Arc;

use crate::server::{ServerState, UserAccount, UsersRepo};

#[async_trait]
impl Authentication<UserAccount, i64, PgPool> for UserAccount {
    async fn load_user(user_id: i64, pool: Option<&PgPool>) -> Result<UserAccount, anyhow::Error> {
        let pool = pool.unwrap();

        // User::get_user(userid, pool)
        //     .await
        //     .ok_or_else(|| anyhow::anyhow!("Could not load user"))
        UsersRepo::get_by_id(user_id, pool)
            .await
            .ok_or_else(|| anyhow::anyhow!("Could not load user"))
    }

    fn is_authenticated(&self) -> bool {
        !self.anonymous
    }

    fn is_active(&self) -> bool {
        !self.anonymous
    }

    fn is_anonymous(&self) -> bool {
        self.anonymous
    }
}

#[async_trait]
impl HasPermission<PgPool> for UserAccount {
    async fn has(&self, perm: &str, _pool: &Option<&PgPool>) -> bool {
        self.permissions.contains(&perm.to_string())
    }
}

// impl User {
//     pub async fn get_user(id: i64, pool: &PgPool) -> Option<Self> {
//         let sqluser = sqlx::query_as::<_, SqlUser>("SELECT * FROM users WHERE id = $1")
//             .bind(id)
//             .fetch_one(pool)
//             .await
//             .ok()?;

//         let sql_user_perms =
//             sqlx::query_as::<_, SqlPermissionTokens>("SELECT permission FROM users_permissions WHERE user_id = $1;")
//                 .bind(id)
//                 .fetch_all(pool)
//                 .await
//                 .ok()?;

//         Some(sqluser.into_user(Some(sql_user_perms)))
//     }
// }

// #[derive(sqlx::FromRow, Clone)]
// pub struct SqlUser {
//     pub id: i32,
//     pub anonymous: bool,
//     pub username: String,
// }

// impl SqlUser {
//     pub fn into_user(self, sql_user_perms: Option<Vec<SqlPermissionTokens>>) -> User {
//         User {
//             id: self.id,
//             anonymous: self.anonymous,
//             username: self.username,
//             permissions: if let Some(user_perms) = sql_user_perms {
//                 user_perms.into_iter().map(|x| x.token).collect::<HashSet<String>>()
//             } else {
//                 HashSet::<String>::new()
//             },
//         }
//     }
// }

pub struct Session(
    /// Auth Session
    pub axum_session_auth::AuthSession<UserAccount, i64, SessionPgPool, PgPool>,
    /// Users Repository
    pub Arc<UsersRepo>,
);

impl std::ops::Deref for Session {
    type Target = axum_session_auth::AuthSession<UserAccount, i64, SessionPgPool, PgPool>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Session {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug)]
pub struct AuthSessionLayerNotFound;

impl std::fmt::Display for AuthSessionLayerNotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AuthSessionLayer was not found")
    }
}

impl std::error::Error for AuthSessionLayerNotFound {}

impl IntoResponse for AuthSessionLayerNotFound {
    fn into_response(self) -> Response {
        (
            http::status::StatusCode::INTERNAL_SERVER_ERROR,
            "AuthSessionLayer was not found",
        )
            .into_response()
    }
}

#[async_trait]
impl<S: std::marker::Sync + std::marker::Send> axum::extract::FromRequestParts<S> for Session {
    type Rejection = AuthSessionLayerNotFound;

    async fn from_request_parts(parts: &mut http::request::Parts, state: &S) -> Result<Self, Self::Rejection> {
        axum_session_auth::AuthSession::<UserAccount, i64, SessionPgPool, PgPool>::from_request_parts(parts, state)
            .await
            .map(|auth_session| {
                let ss = parts.extensions.get::<ServerState>().unwrap();
                let ur = ss.users_repo.clone();
                Session(auth_session, ur)
            })
            .map_err(|_| AuthSessionLayerNotFound)
    }
}
