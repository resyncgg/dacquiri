use dacquiri::prelude::*;
use crate::attributes::*;
use crate::models::User;

#[entitlement(AccountIsEnabled)]
pub trait EnabledAccount {
    fn change_name(&mut self, new_name: impl Into<String>) {
        self.get_subject_mut().change_name(new_name.into());
    }
}

#[entitlement(AccountIsEnabled, AccountIsMatured)]
pub trait MaturedAccount {
    fn post_message(&self, message: impl Into<String>) {
        let user: &User = self.get_subject();

        println!("[{}] {}", user.get_account_name(), message.into());
    }
}

#[entitlement(AccountIsAdmin)]
pub trait AdminAccount {
    fn do_nothing(&self) {
        println!("Hello, world!");
    }

    fn enable_account(&self, user: &mut User) {
        user.set_enabled(true);
    }
}