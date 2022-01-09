use dacquiri::prelude::*;
use crate::principal::{Team, User};

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

#[grant(TeamMember)]
fn check_team_perm(user: User, team: Team) -> GrantResult<()> {
    Ok(())
}

#[requirement(ChangeName, AccountEnabled)]
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
        let team_one: &Team = HasGrant::<TeamMember<"Check1">, _>::get_resource(self);
        let team_two: &Team = HasGrant::<TeamMember<"Check2">, _>::get_resource(self);

        println!("Team 1 '{}' and Team 2 '{}'", team_one.get_name(), team_two.get_name());
    }
}