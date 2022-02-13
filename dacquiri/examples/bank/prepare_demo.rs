use crate::{
    ACCOUNT_ONE_PASSWORD,
    ACCOUNT_TWO_PASSWORD,
    ADMIN_PASSWORD
};
use dacquiri::prelude::*;
use crate::bank::*;
use crate::bank::attributes::*;
use crate::bank::policies::*;

/// Creates a bank, bank admin, two accounts, and gives those accounts some money
pub fn prepare_demo() -> (BankHandle, BankAdmin, AccountID, AccountID) {
    let (bank_admin, bank) = Bank::create_bank(ADMIN_PASSWORD);

    let (account_one, account_two) = create_test_accounts(bank.clone(), bank_admin.clone())
        .expect("Failed to create test accounts.");

    deposit_money(bank.clone(), &account_one, 100)
        .expect("Failed to deposit money into account one.");

    deposit_money(bank.clone(), &account_two, 200)
        .expect("Failed to deposit money into account two.");

    (bank, bank_admin, account_one, account_two)
}

fn create_test_accounts(bank: BankHandle, bank_admin: BankAdmin) -> BankResult<(AccountID, AccountID)> {
    let mut account_creator = bank_admin
        .into_entity::<"admin">()
        .add_entity::<_, "bank">(bank)?
        .constrain_with_resource::<AssignedBankAdmin, "admin", "bank">()?
        .constrain_with_context::<AdminAuthorized, "admin">(ADMIN_PASSWORD)?;

    let account_one = account_creator.create_account(ACCOUNT_ONE_PASSWORD);
    let account_two = account_creator.create_account(ACCOUNT_TWO_PASSWORD);

    Ok((account_one, account_two))
}

fn deposit_money(bank: BankHandle, account: &AccountID, amount: u64) -> BankResult<()> {
    let mut depositor = (*account)
        .into_entity::<"account">()
        .add_entity::<_, "bank">(bank)?
        .constrain_with_resource::<NotFrozen, "account", "bank">()?;

    let _ = depositor.deposit(amount)?;

    Ok(())
}