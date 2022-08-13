use thiserror::Error;
use dacquiri::prelude::ConstraintError;

#[derive(Error, Debug)]
pub enum AuthorizationError {
    #[error("User is not enabled.")]
    UserNotEnabled,
    #[error("General error occurred.")]
    GeneralError,
}

impl From<ConstraintError> for AuthorizationError {
    fn from(_: ConstraintError) -> Self {
        AuthorizationError::GeneralError
    }
}