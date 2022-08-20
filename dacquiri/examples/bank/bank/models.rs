use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::{Mutex, MutexGuard};
use rand::Rng;
use crate::bank::error::{BankError, BankResult};


#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct AccountID(u64);

impl From<u64> for AccountID {
    fn from(value: u64) -> Self {
        AccountID(value)
    }
}

pub(in crate::bank) struct Account {
    balance: u64,
    password: String,
    frozen: bool
}

impl Account {
    pub(in crate::bank) fn is_frozen(&self) -> bool {
        self.frozen
    }

    pub(in crate::bank) fn check_password(&self, password: impl Into<String>) -> bool {
        // this is not designed to be secure - this is vulnerable to timing attacks

        self.password == password.into()
    }
}

#[derive(Clone)]
pub struct BankAdmin {
    admin_id: u128,
    admin_password: String,
}

impl BankAdmin {
    pub(in crate::bank) fn check_password(&self, password: impl Into<String>) -> bool {
        // this is not design to be secure - this is vulnerable to timing attacks

        self.admin_password == password.into()
    }

    pub(in crate::bank) fn get_admin_id(&self) -> u128 {
        self.admin_id
    }
}

pub struct Bank {
    admin_id: u128,
    accounts: HashMap<AccountID, Account>,
}

impl Bank {
    pub fn create_bank(admin_password: impl Into<String>) -> (BankAdmin, BankHandle) {
        let admin_id = rand::thread_rng().gen();

        let bank_admin = BankAdmin {
            admin_id,
            admin_password: admin_password.into()
        };

        let bank = Bank {
            admin_id,
            accounts: HashMap::new()
        };

        let inner = Arc::new(Mutex::new(bank));

        let handle = BankHandle {
            inner
        };

        (bank_admin, handle)
    }



    pub(in crate::bank) fn open_account(&mut self, password: impl Into<String>) -> AccountID {
        const EMPTY_BALANCE: u64 = 0;

        let next_id = self
            .accounts
            .keys()
            .map(|account_id| account_id.0)
            .max()
            .unwrap_or(0); // first account

        let account_id = (next_id + 1).into();
        let account = Account {
            balance: EMPTY_BALANCE,
            password: password.into(),
            frozen: false
        };

        self.accounts.insert(account_id, account);

        account_id
    }

    pub(in crate::bank) fn close_account(&mut self, account_id: AccountID) -> BankResult<u64> {
        match self.accounts.remove(&account_id) {
            Some(Account { balance, .. }) => Ok(balance),
            None => Err(BankError::AccountDoesntExist)
        }
    }

    pub(in crate::bank) fn withdraw(&mut self, account_id: AccountID, amount: u64) -> BankResult<u64> {
        match self.accounts.get_mut(&account_id) {
            Some(Account { balance, .. }) if *balance >= amount => {
                *balance -= amount;

                Ok(amount)
            },
            Some(_) => Err(BankError::InsufficientBalance),
            None => Err(BankError::AccountDoesntExist)
        }
    }

    pub(in crate::bank) fn deposit(&mut self, account_id: AccountID, amount: u64) -> BankResult<()> {
        let balance = match self.accounts.get_mut(&account_id) {
            Some(Account { balance, .. }) => Ok(balance),
            None => Err(BankError::AccountDoesntExist)
        }?;

        let new_balance = balance
            .checked_add(amount)
            .ok_or(BankError::DepositFailed)?;

        *balance = new_balance;

        Ok(())
    }

    pub(in crate::bank) fn get_account(&self, account_id: &AccountID) -> BankResult<&Account> {
        self
            .accounts
            .get(account_id)
            .ok_or(BankError::AccountDoesntExist)
    }

    pub(in crate::bank) fn get_admin_account_id(&self) -> u128 {
        self.admin_id
    }
}

#[derive(Clone)]
pub struct BankHandle {
    inner: Arc<Mutex<Bank>>
}

impl BankHandle {
    pub fn lock(&self) -> MutexGuard<'_, Bank> {
        self.inner.lock()
    }
}