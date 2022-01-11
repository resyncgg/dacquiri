#![feature(generic_arg_infer)]

use dacquiri::prelude::*;

#[derive(Principal)]
struct User;

struct PermissionOne;
impl Grant for PermissionOne {
    type Principal = User;

    fn new_with_resource(_: Self::Resource) -> Self { Self }
    fn get_resource(&self) -> &Self::Resource { &() }
    fn check_grant(_: &Self::Principal, _: &Self::Resource) -> Result<(), Self::Error> { Ok(()) }
}

struct PermissionTwo;
impl Grant for PermissionTwo {
    type Principal = User;

    fn new_with_resource(_: Self::Resource) -> Self { Self }
    fn get_resource(&self) -> &Self::Resource { &() }
    fn check_grant(_: &Self::Principal, _: &Self::Resource) -> Result<(), Self::Error> { Ok(()) }
}

fn main() {
    let user = User;

    let grant = user.try_grant_with_resource_and_context_async::<PermissionOne, _>(())
        .expect("Missing permission one.");

    guarded_function(grant);
}

fn guarded_function(caller: impl HasGrant<PermissionOne> + HasGrant<PermissionTwo>) {
    println!("User has both permission one and permission two");
}