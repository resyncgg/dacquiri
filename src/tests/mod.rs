use crate::grant::{Grant, Grantable, PrincipalT};
use crate::tests::helpers::grants::add_user::{AddUserToTeam, AddUserToTeamGrant};
use crate::tests::helpers::{Team, User};
use crate::tests::helpers::grants::admin::{AdminOne, AdminTwo, BigAdmin};
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

    let idkman = user.try_grant::<AdminOne, _>(())?
                                    .try_grant::<AdminTwo, _>(())?;

    kekw(idkman)
}

fn kekw(caller: impl BigAdmin) -> Result<(), String> {
    caller.mogul_moves();

    Ok(())
}