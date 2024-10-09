#[derive(Debug, thiserror::Error)]
pub enum CategoryError {
    #[error("Category does not exist.")]
    NotFound,
    #[error("Category already exist.")]
    DuplicateCategory,
    #[error("Something went wrong.")]
    DatabaseError(#[from] surrealdb::Error),
    #[error("Validation check failed for Category.")]
    ValidationFailed(#[from] validator::ValidationErrors),
}
