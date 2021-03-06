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

fn main() {
    let user = User;

    guarded_function(user);
}

fn guarded_function(caller: impl HasGrant<PermissionOne>) {
    println!("User has PermissionOne grant.");
}