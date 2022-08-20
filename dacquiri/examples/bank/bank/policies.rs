use dacquiri::prelude::*;
use crate::bank::{AccountID,  BankHandle, BankAdmin, BankResult};
use crate::bank::attributes::*;

#[policy(
    entities = (
        admin: BankAdmin,
        bank: BankHandle
    ),
    guard = (
        admin is AdminAuthorized,
        admin is AssignedBankAdmin for bank
    )
)]
pub trait AuthorizedAdminPolicy {
    fn create_account(&mut self, password: impl Into<String>) -> AccountID {
        let the_bank: &BankHandle = self.get_entity::<_, bank>();

        the_bank
            .lock()
            .open_account(password)
    }
}

#[policy(
    entities = (
        account: AccountID,
        bank: BankHandle
    ),
    guard = (
        account is NotFrozen for bank
    )
)]
pub trait ActiveAccountPolicy {
    fn deposit(&mut self, amount: u64) -> BankResult<()> {
        let my_account_id: AccountID = *self.get_entity::<_, account>();
        let the_bank: &BankHandle = self.get_entity::<_, bank>();

        the_bank
            .lock()
            .deposit(my_account_id, amount)
    }
}

#[policy(
    entities = (
        account: AccountID,
        bank: BankHandle
    ),
    guard = (
        ActiveAccountPolicy(account, bank),
        account is Authenticated for bank,
    )
)]
pub trait AuthenticatedAccountPolicy {
    fn withdraw(&mut self, amount: u64) -> BankResult<u64> {
        let my_account_id: AccountID = *self.get_entity::<_, account>();
        let the_bank: &BankHandle = self.get_entity::<_, bank>();

        the_bank
            .lock()
            .withdraw(my_account_id, amount)
    }
}

#[policy(
    entities = (
        account: AccountID,
        bank: BankHandle,
        admin: BankAdmin
    ),
    guard = (
        AuthenticatedAccountPolicy(account, bank),
        AuthorizedAdminPolicy(admin, bank)
    )
)]
pub trait AccountClosingPolicy {
    fn close_account(self) -> BankResult<u64> {
        let inner_bank: &BankHandle = self.get_entity::<_, bank>();
        let subject_account: &AccountID = self.get_entity::<_, account>();

        let cash = inner_bank
            .lock()
            .close_account(*subject_account)?;

        Ok(cash)
    }
}