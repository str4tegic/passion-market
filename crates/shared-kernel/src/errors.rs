#[derive(Debug, thiserror::Error)]
pub enum DateTimeError {
    #[error("{0}")]
    ValidationError(String),
}
