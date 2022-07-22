#[derive(Debug, thiserror::Error)]
pub enum AxumError {
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    AxumJsonRejection(#[from] axum::extract::rejection::JsonRejection),
}
