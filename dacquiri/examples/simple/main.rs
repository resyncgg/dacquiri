#![allow(incomplete_features)]
#![feature(generic_associated_types)]
#![feature(adt_const_params)]
#![feature(generic_arg_infer)]
#![feature(explicit_generic_args_with_impl_trait)]

use dacquiri::prelude::{AcquireAttribute, AcquireAttributeWithContext, AcquireAttributeWithResourceAndContext, ConstraintT, InitializeConstraint};
use crate::models::{Team, User};
use crate::attributes::{
    UserIsEnabled,
    TeamIsEnabled
};

mod models;
mod attributes;
mod entitlements;

#[tokio::main]
async fn main() -> Result<(), String> {
    let admin = User::new("d0nut", 0);
    let team = Team::new("public", &admin);

    let constraints = admin
        .begin_constraint::<"User">()
        .constrain::<UserIsEnabled, "User">()?
        .add_element::<"Team">(team)?
        .constrain::<TeamIsEnabled, "Team">()?;

    // let nope: String = constraints;

    Ok(())
}