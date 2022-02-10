use dacquiri::prelude::*;
use crate::models::{Team, User};

#[attribute(EnabledUser)]
pub fn check_user_is_enabled(user: &User) -> AttributeResult<String> {
    if user.is_enabled() {
        Ok(())
    } else {
        Err(format!("User not enabled"))
    }
}

#[attribute(EnabledTeam)]
pub fn check_team_is_enabled(team: &Team) -> AttributeResult<String> {
    if team.is_enabled() {
        Ok(())
    } else {
        Err(format!("Team not enabled"))
    }
}

#[attribute(MemberOfTeam)]
pub fn check_user_is_on_team(_: &User, _: &Team) -> AttributeResult<String> {
    Ok(())
}