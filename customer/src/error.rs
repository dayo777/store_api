#[derive(thiserror::Error, Debug)]
pub enum CustomerError {
    #[error("Duplicate email address or name already exist.")]
    DuplicateEmailAddress,
    #[error("Customer does not exist.")]
    CustomerDoesNotExist,
    #[error("Something went wrong. Unable to retrieve data")]
    InternalError(#[from] surrealdb::Error),
    #[error("Validation check failed.")]
    ValidationFailed(#[from] validator::ValidationErrors),
    #[error("Old & new email addresses are the same.")]
    SameEmailError,
    #[error("Both passwords contain the same value.")]
    PasswordMatchError,
    #[error("Wrong password.")]
    PasswordDoesNotExist,
    #[error("Error encrypting password.")]
    PasswordEncryptionError(#[from] bcrypt::BcryptError),
}
