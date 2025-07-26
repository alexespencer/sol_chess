use thiserror::Error;

#[derive(Debug, Error)]
pub enum SError {
    #[error("invalid board")]
    InvalidBoard,
    #[error("invalid notation")]
    InvalidNotation,
}
