#![feature(bool_to_option)]
#![feature(generic_arg_infer)]
#![feature(trait_alias)]
#![feature(adt_const_params)]

use dacquiri::prelude::*;
use crate::grants::{AccountEnabled, CanChangeName, ChangeName, PrintBothTeamNames, TeamMember};
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
        .try_grant::<AccountEnabled, _>(())?
        .try_grant::<ChangeName, _>(())?;

    print_name(&mut chain);

    let mut new_chain = chain
        .try_grant::<TeamMember<"Check1">, _>(team_one)?
        .try_grant::<TeamMember<"Check2">, _>(team_two)?;

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