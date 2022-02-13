use dacquiri::prelude::*;
use crate::bank::{AccountID, BankAdmin, BankError};
use crate::bank::models::BankHandle;

#[attribute(AdminAuthorized)]
pub fn check_admin_authorized_action(admin: &BankAdmin, _: &(), admin_password: &str) -> AttributeResult<BankError> {
    if admin.check_password(admin_password) {
        Ok(())
    } else {
        Err(BankError::IncorrectBankAdminPassword)
    }
}

#[attribute(AssignedBankAdmin)]
pub fn check_admin_is_assigned_to_bank(admin: &BankAdmin, bank: &BankHandle) -> AttributeResult<BankError> {
    if bank.lock().get_admin_account_id() == admin.get_admin_id() {
        Ok(())
    } else {
        Err(BankError::IncorrectBankAdminPassword)
    }
}

#[attribute(NotFrozen)]
pub fn check_account_is_not_frozen(account_id: &AccountID, bank: &BankHandle) -> AttributeResult<BankError> {
    match bank.lock().get_account(account_id)? {
        account if !account.is_frozen() => Ok(()),
        _ => Err(BankError::AccountIsFrozen)
    }
}

#[attribute(Authenticated)]
pub fn check_caller_has_account_password(
    account_id: &AccountID,
    bank: &BankHandle,
    password: &str
)  -> AttributeResult<BankError> {
    match bank.lock().get_account(account_id)? {
        account if account.check_password(password) => Ok(()),
        _ => Err(BankError::IncorrectAccountPassword),
    }
}