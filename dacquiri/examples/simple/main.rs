#![allow(incomplete_features)]
#![feature(generic_associated_types)]
#![feature(adt_const_params)]
#![feature(generic_arg_infer)]

use dacquiri::prelude::Grantable;
use crate::models::User;
use crate::attributes::*;
use crate::entitlements::{
    MaturedAccount,
    AdminAccount
};

mod models;
mod attributes;
mod error;
mod entitlements;

#[tokio::main]
async fn main() {
    let admin_user = User::new("d0nut", 0);
    let mut random_user = User::new("insanitybit", 1);

    /*
        Check that the admin_user is *actually* an admin and then call the `do_nothing()` function.
     */
    let checked_admin_user = admin_user
        .try_grant::<AccountIsAdmin>()
        .expect("User was not an admin.");
    checked_admin_user.do_nothing();
    // we must enable the "random_user" account
    checked_admin_user.enable_account(&mut random_user);

    let checked_random_user = random_user
        .try_grant::<AccountIsEnabled>()
        .expect("User is not enabled.")
        .try_grant::<AccountIsMatured>()
        // we will panic here because our account was *just* created!
        // however, deleting this try_grant will result in a compiler error
        .expect("User account is not at least 30 days old.");

    guarded_function(checked_random_user, "Hello, world :)");
}

fn guarded_function(user: impl MaturedAccount, message: impl Into<String>) {
    user.post_message(message);
}