use shared_kernel::errors::DateTimeError;

#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("not found")]
    NotFound,
    #[error("{0}")]
    Conflict(String),
    #[error("{0}")]
    ValidationError(String),
    #[error("unauthorized")]
    Unauthorized,
    #[error("forbidden")]
    Forbidden,
}

impl From<DateTimeError> for DomainError {
    fn from(e: DateTimeError) -> Self {
        match e {
            DateTimeError::ValidationError(msg) => DomainError::ValidationError(msg),
        }
    }
}
