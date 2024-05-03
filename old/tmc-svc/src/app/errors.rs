use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    //
    #[error("internal error")]
    InternalErr,

    #[error("unauthorized: {0}")]
    Unauthorized(String),
}

impl From<sqlx::Error> for AppError {
    //

    fn from(err: sqlx::Error) -> Self {
        log::error!("Got db error: '{}'.", err);
        // TODO: For now, all database related errors are classified as internal errors.
        AppError::InternalErr
    }
}
