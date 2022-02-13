use thiserror::Error;
use dacquiri::prelude::ConstraintError;

pub type BankResult<T> = Result<T, BankError>;

#[derive(Debug, Error)]
pub enum BankError {
    #[error("An error occurred.")]
    GeneralError,
    #[error("The specified account doesn't exist.")]
    AccountDoesntExist,
    #[error("The specified account balance is too low.")]
    InsufficientBalance,
    #[error("The deposit failed.")]
    DepositFailed,
    #[error("The specified account is frozen.")]
    AccountIsFrozen,
    #[error("The supplied admin password was incorrect.")]
    IncorrectBankAdminPassword,
    #[error("The supplied password was incorrect for the specified account.")]
    IncorrectAccountPassword,
}

impl From<ConstraintError> for BankError {
    fn from(_: ConstraintError) -> Self {
        BankError::GeneralError
    }
}