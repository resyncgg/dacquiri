#![allow(dead_code)]
#![allow(incomplete_features)]
#![feature(generic_associated_types)]
#![feature(adt_const_params)]
#![feature(generic_arg_infer)]
#![feature(explicit_generic_args_with_impl_trait)]

use dacquiri::prelude::{AcquireAttribute, AcquireAttributeWithContext, ConstraintT, InitializeConstraint};
use crate::models::{Team, User};
use crate::attributes::{
    EnabledUser,
    EnabledTeam,
    MemberOfTeam
};
use crate::policies::MultiTeamMember;

mod models;
mod attributes;
mod policies;

fn main() -> Result<(), String> {
    let user = User::new("d0nut", 0);
    let team_a = Team::new("TeamA", &user);
    let team_b = Team::new("TeamB", &user);

    let multi_team_member = user
        .begin_constraint::<"user">()
        .add_entity::<"team_a">(team_a)?
        .add_entity::<"team_b">(team_b)?
        .constrain::<EnabledUser, "user">()?
        .constrain::<EnabledTeam, "team_a">()?
        .constrain::<EnabledTeam, "team_b">()?
        .constrain_with_resource::<MemberOfTeam, "user", "team_a">()?
        .constrain_with_resource::<MemberOfTeam, "user", "team_b">()?;

    do_transfer_between_teams(multi_team_member)
}

fn do_transfer_between_teams(caller: impl MultiTeamMember) -> Result<(), String> {
    caller.transfer_data();

    Ok(())
}