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
use crate::attributes::*;
use crate::error::AuthorizationError;
use crate::models::User;
use crate::policies::*;

fn main() -> AttributeResult<AuthorizationError> {
    let user = User::new("d0nut", true);

    let caller = user
        .into_entity::<"user">()
        .check_if_user_is_enabled::<"user">()?;

    guarded(caller)
}

fn guarded(caller: impl EnabledUserPolicy) -> AttributeResult<AuthorizationError> {
    caller.print_name();

    Ok(())
}