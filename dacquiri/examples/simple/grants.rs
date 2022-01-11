use dacquiri::prelude::*;
use crate::principal::{Team, User};

#[grant(AccountEnabled)]
fn check_account_enabled(user: &User) -> GrantResult<()> {
    user.is_enabled()
        .then_some(())
        .ok_or(())
}

#[grant(ChangeName)]
fn check_change_name(_: &User) -> GrantResult<()> {
    Ok(())
}

#[grant(TeamMember)]
fn check_team_perm(_: &User, _: &Team) -> GrantResult<()> {
    Ok(())
}

#[grant(ContextGrant)]
fn check_context_function(_: &User, _: &(), foo: String) -> GrantResult<()> {
    println!("Logging inside the check function: {}", foo);
    Ok(())
}

#[grant(MyAsyncGrant)]
async fn check_async_grant(_: &User) -> GrantResult<()> {
    println!("We've successfully called an async grant check :D");

    Ok(())
}

#[grant(MyAsyncGrantWContext)]
async fn check_async_grant_with_context(_: &User, _: &(), context: &'ctx String) -> GrantResult<()> {
    println!("async grant w/ resource number 1: {}", context);

    Ok(())
}

#[grant(MyAsyncGrantWContext2)]
async fn check_async_grant_with_context2(_: &User, _: &(), context: &'ctx String) -> GrantResult<()> {
    println!("async grant w/ resource number 2: {}", context);

    Ok(())
}

#[grant(AsyncGrantWithTupleContext)]
async fn complex_af(_: &User, _: &(), context: (&'ctx String, &'ctx mut String)) -> GrantResult<()> {
    println!("Provided two elements of context: (\"{}\", \"{}\")", context.0, context.1);

    Ok(())
}

#[requirement(ChangeName, AccountEnabled)]
#[async_trait::async_trait]
pub trait CanChangeName {
    fn change_name(&mut self, name: impl Into<String>) {
        self.get_principal_mut().set_name(name);
    }
}

#[requirement(
    AccountEnabled,
    TeamMember as "Check1",
    TeamMember as "Check2",
)]
pub trait PrintBothTeamNames {
    fn print_both_team_names(&self) {
        let team_one: &Team = get_resource!(self as TeamMember["Check1"]);
        let team_two: &Team = get_resource!(self as TeamMember["Check2"]);

        println!("Team 1 '{}' and Team 2 '{}'", team_one.get_name(), team_two.get_name());
    }
}