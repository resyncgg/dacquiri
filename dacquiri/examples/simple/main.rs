#![allow(incomplete_features)]
#![feature(bool_to_option)]
#![feature(generic_arg_infer)]
#![feature(trait_alias)]
#![feature(adt_const_params)]
#![feature(in_band_lifetimes)]
#![feature(generic_associated_types)]

use dacquiri::prelude::*;
use crate::grants::*;
use crate::principal::{Team, User};

mod principal;
mod grants;

#[tokio::main]
async fn main() -> GrantResult<()> {
    let mut user = User::new("d0nut");
    let team_one = Team::new("team 1");
    let team_two = Team::new("team 2");

    // required, otherwise runtime error
    user.enable_account();

    let db_connection = format!("pretend this string is, instead, a database connection");

    let message = format!("Woah");

    let left = format!("left");
    let mut right = format!("right");

    // required, otherwise compilation error
    let mut chain = user
        .try_grant::<AccountEnabled>()?
        .try_grant::<ChangeName>()?
        .try_grant_with_context::<ContextGrant>(db_connection)?
        .try_grant_async::<MyAsyncGrant>().await?
        .try_grant_with_resource_and_context_async::<MyAsyncGrantWContext, _>((), &message).await?
        .try_grant_with_resource_and_context_async::<MyAsyncGrantWContext2, _>((), &message).await?
        .try_grant_with_resource_and_context_async::<AsyncGrantWithTupleContext, _>((), (&left, &mut right)).await?;

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