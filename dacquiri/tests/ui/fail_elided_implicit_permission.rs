#![feature(generic_arg_infer)]

// In this test case, we're going to check both permission up front
// but since guarded_function_one will only enforce that we have PermissionOne
// we'll lose the guarantee that we are granted PermissionTwo.
// if we fail to recheck PermissionTwo, we'll lead to a compilation error.

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

    let both_grants = user.try_grant_with_resource_and_context::<PermissionOne, _>(())
        .expect("Missing permission one.")
        .try_grant_with_resource_and_context::<PermissionTwo, _>(())
        .expect("Missing permission two.");

    guarded_function_one(both_grants);
}

fn guarded_function_one(caller: impl HasGrant<PermissionOne>) {
    println!("User has permission one.");
    guarded_function_two(caller)
}

fn guarded_function_two(caller: impl HasGrant<PermissionOne> + HasGrant<PermissionTwo>) {
    println!("User has permission one and permission two.");
}