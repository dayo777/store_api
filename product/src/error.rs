#[derive(thiserror::Error, Debug)]
pub enum ProductError {
    #[error("Validation check failed.")]
    ValidationError(#[from] validator::ValidationErrors),
    #[error("Something went wrong. Unable to retrieve data.")]
    InternalError(#[from] surrealdb::Error),
    #[error("Product name already exist.")]
    DuplicateProductName,
    #[error("Invalid Category.")]
    InvalidCategory,
    #[error("Product does not exist.")]
    ProductDoesNotExist,
}
