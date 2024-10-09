#[derive(thiserror::Error, Debug)]
pub(crate) enum OrderError {
    #[error(transparent)]
    InternalError(#[from] surrealdb::Error),
    #[error("Invalid Order.")]
    InvalidOrder,
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),
    #[error(transparent)]
    CustomerError(#[from] customer::CustomerError),
    // #[error("Issue building Order-items. Unable to retrieve products.")]
    // ProductError,
}
