#![allow(incomplete_features)]
#![feature(bool_to_option)]
#![feature(generic_arg_infer)]
#![feature(trait_alias)]
#![feature(adt_const_params)]

use dacquiri::prelude::*;
use crate::grants::{AccountEnabled, CanChangeName, ChangeName, PrintBothTeamNames, TeamMember, ContextGrant};
use crate::principal::{Team, User};

mod principal;
mod grants;

fn main() -> GrantResult<()> {
    let mut user = User::new("d0nut");
    let team_one = Team::new("team 1");
    let team_two = Team::new("team 2");

    // required, otherwise runtime error
    user.enable_account();

    // required, otherwise compilation error
    let mut chain = user
        .try_grant::<AccountEnabled>()?
        .try_grant::<ChangeName>()?
        .try_grant_with_context::<ContextGrant>(format!("Woah!!"))?;

    print_name(&mut chain);

    let new_chain = chain
        .try_grant_with_resource::<TeamMember<"Check1">, _>(team_one)?
        .try_grant_with_resource::<TeamMember<"Check2">, _>(team_two)?;

    do_the_thing_zhu_li(&new_chain);

    Ok(())
}

fn print_name(user: &mut impl CanChangeName) {
    println!("My name is: {}", user.get_principal().get_name());

    user.change_name("new_name");

    println!("My new name is: {}", user.get_principal().get_name());
}

fn do_the_thing_zhu_li(user: &impl PrintBothTeamNames) {
    user.print_both_team_names();
}