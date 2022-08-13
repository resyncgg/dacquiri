#![deny(warnings)]
#![allow(incomplete_features)]
#![feature(generic_associated_types)]
#![feature(adt_const_params)]
#![feature(generic_arg_infer)]
#![feature(rustc_attrs)]
#![feature(marker_trait_attr)]

mod models;
mod attributes;
mod error;
mod policies;

use dacquiri::prelude::*;
use crate::attributes::Enabled;
use crate::error::AuthorizationError;
use crate::models::User;
use crate::policies::EnabledUserPolicy;

fn main() -> Result<(), AuthorizationError> {
    let user = User::new("d0nut", true);
    let enabled_user = user
        .into_entity::<"user">()
        .prove::<Enabled, "user">()?;

    enabled_user.print_name();

    Ok(())
}