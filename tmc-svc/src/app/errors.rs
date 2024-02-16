use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    //
    #[error("internal error")]
    InternalErr,
}

impl From<sqlx::Error> for AppError {
    //
    fn from(_error: sqlx::Error) -> Self {
        AppError::InternalErr
    }
}
