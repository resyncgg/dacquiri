use crate::{Grant, Grantable, PrincipalT};
use crate::tests::helpers::grants::add_user::{AddUserToTeam, AddUserToTeamGrant};
use crate::tests::helpers::{Team, User};
use crate::tests::helpers::grants::change_name::{ChangeName, ChangeNameGrant};

mod helpers;

const GRANT_CHECK_A: &'static str = "GrantCheck::A";
const GRANT_CHECK_B: &'static str = "GrantCheck::B";

#[test]
fn test() -> Result<(), String> {
    let mut user = User::new("d0nut");
    let team_one = Team::new("Team One", 123);
    let team_two = Team::new("Team Two", 321);

    user.add_grant(AddUserToTeam::<"_">::name())
        .add_team_id(team_one.get_team_id())
        .add_team_id(team_two.get_team_id());

    println!("Our original name is: {}", user.get_name());

    let authorized_user = user
        .try_grant::<AddUserToTeam<"Check1">, _>(team_one.clone())?
        .try_grant::<ChangeName, _>(())?;

    do_change_name(authorized_user, "karimpwnz", team_one, team_two);

    Ok(())
}

fn do_change_name(
    mut caller: impl ChangeNameGrant
                + Grantable<User, Team>,
    name: impl Into<String>,
    team1: Team,
    team2: Team
) {
    println!("We're about to change our name.");
    caller.change_name(name);
    println!("Our new name is: {}", caller.get_principal().get_name());

    let has_additional_permissions = caller
        .try_grant::<AddUserToTeam<"Check1">, _>(team1)
        .expect("team1")
        .try_grant::<AddUserToTeam<"Check2">, _>(team2)
        .expect("team2");

    add_user_to_two_teams(has_additional_permissions);
}

fn add_user_to_two_teams(
    caller: impl AddUserToTeamGrant<"Check1">
                + AddUserToTeamGrant<"Check2">
) {
    println!("About to add the user to two different teams.");
    AddUserToTeamGrant::<"Check1">::add_user(&caller);
    AddUserToTeamGrant::<"Check2">::add_user(&caller);
}