use crate::app::errors::AppError;

pub type ApiResult<T, E = AppError> = std::result::Result<T, E>;
