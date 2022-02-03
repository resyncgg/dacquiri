use crate::attributes::*;
use crate::models::{
    User,
    Team
};
use dacquiri::prelude::entitlement;

#[entitlement(
    User as "User",
    Team as "TeamA",
    Team as "TeamB",
    constraints = (
        "User" is UserIsEnabled,
        "TeamA" is TeamIsEnabled,
        "TeamB" is TeamIsEnabled,
        "User" is IsTeamMember for "TeamA",
        "User" is IsTeamMember for "TeamB"
    )
)]
pub trait TestEntitlement {

}