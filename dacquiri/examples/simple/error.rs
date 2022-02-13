use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthorizationError {
    #[error("User is not enabled.")]
    UserNotEnabled
}