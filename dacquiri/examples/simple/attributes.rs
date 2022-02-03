use dacquiri::prelude::*;
use crate::models::{Team, User};

#[attribute(UserIsEnabled)]
pub fn check_user_is_enabled(user: &User) -> AttributeResult<String> {
    if user.is_enabled() {
        Ok(())
    } else {
        Err(format!("User not enabled"))
    }
}

#[attribute(TeamIsEnabled)]
pub fn check_team_is_enabled(team: &Team) -> AttributeResult<String> {
    if team.is_enabled() {
        Ok(())
    } else {
        Err(format!("Team not enabled"))
    }
}