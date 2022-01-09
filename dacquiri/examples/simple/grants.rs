use dacquiri::prelude::*;
use crate::principal::User;

#[grant(AccountEnabled)]
fn check_account_enabled(user: User) -> GrantResult<()> {
    user.is_enabled()
        .then_some(())
        .ok_or(())
}

#[grant(ChangeName)]
fn check_change_name(_: User) -> GrantResult<()> {
    Ok(())
}

#[requirement(ChangeName, AccountEnabled)]
pub trait CanChangeName {
    fn change_name(&mut self, name: impl Into<String>) {
        self.get_principal_mut().set_name(name);
    }
}
