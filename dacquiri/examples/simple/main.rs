#![feature(bool_to_option)]
#![feature(generic_arg_infer)]
#![feature(trait_alias)]

use dacquiri::prelude::*;
use crate::grants::{AccountEnabled, CanChangeName, ChangeName};
use crate::principal::User;

mod principal;
mod grants;

fn main() -> Result<(), ()> {
    let mut user = User::new("d0nut");

    // required, otherwise runtime error
    user.enable_account();

    // required, otherwise compilation error
    let mut chain = user
        .try_grant::<AccountEnabled, _>(())?
        .try_grant::<ChangeName, _>(())?;

    print_name(&mut chain);

    Ok(())
}

fn print_name(user: &mut impl CanChangeName) {
    println!("My name is: {}", user.get_principal().get_name());

    user.change_name("new_name");

    println!("My new name is: {}", user.get_principal().get_name());
}

