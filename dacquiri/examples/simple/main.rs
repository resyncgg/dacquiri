// #![deny(warnings)]
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
use crate::attributes::{
    Admin,
    Enabled,
    Hon
};
use crate::error::AuthorizationError;
use crate::models::User;
use crate::policies::EnabledUserPolicy;

fn main() -> Result<(), AuthorizationError> {
    let user = User::new("d0nut", true);
    let message = String::from("hon");
    let enabled_user = user
        .into_entity::<"user">()
        .add_entity::<_, "message">(message)?
        .prove::<Enabled, "user">()?
        .prove::<Hon, "message">()?;

    enabled_user.print_name();

    let admin = User::new("admin", false);
    let admin_user = admin
        .into_entity::<"user">()
        .prove::<Admin, "user">()?;



    EnabledUserPolicy::<"user">::print_name(&admin_user);

    Ok(())
}