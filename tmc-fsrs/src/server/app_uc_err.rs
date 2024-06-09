//! This module provides:
//! - `AppError` - to abstract any infrastructure and low-level errors (like database related ones)
//!                and convert them into an app (domain) specific ones.
//! - `AppUseCase`s - relevant for the proper conversion from a low-level error to a higher (`AppError`) one.
//!
//! Different cases are considered such as:
//! - for a database error with code 23505
//!   (see its [postgres specifics](https://www.postgresql.org/docs/9.3/errcodes-appendix.html))

use thiserror::Error;

#[derive(Debug)]
pub enum AppUseCase {
    UserRegistration,
    UserLogin,
    GetUserProfile,
}

pub type AppResult<T> = std::result::Result<T, AppError>;

#[derive(Debug, Error)]
pub enum AppError {
    //
    #[error("{0} already exists")]
    AlreadyExists(String),

    #[error("")]
    Ignorable,

    #[error("internal error")]
    InternalErr,

    // #[error("invalid request: {0}")]
    // InvalidRequest(String),
    //
    #[error("{0} not found")]
    NotFound(String),

    #[error("unauthorized: {0}")]
    Unauthorized(String),
}
