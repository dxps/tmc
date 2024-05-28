use std::sync::Arc;

#[cfg(feature = "server")]
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
};
#[cfg(feature = "server")]
use http::{request::Parts, StatusCode};
#[cfg(feature = "server")]
use sqlx::PgPool;

use super::{auth::AuthMgr, UsersRepo};

#[cfg(feature = "server")]
#[derive(Clone)]
pub struct ServerState {
    pub users_repo: Arc<UsersRepo>,
    pub auth_mgr: Arc<AuthMgr>,
    pub db_pool: Arc<PgPool>,
}

impl ServerState {
    pub fn new(db_pool: Arc<PgPool>) -> Self {
        let users_repo = Arc::new(UsersRepo::new(db_pool.clone()));
        Self {
            users_repo: users_repo.clone(),
            auth_mgr: Arc::new(AuthMgr::new(users_repo)),
            db_pool,
        }
    }
}

#[cfg(feature = "server")]
#[async_trait]
impl<S> FromRequestParts<S> for ServerState
where
    Self: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        Ok(Self::from_ref(state))
    }
}
