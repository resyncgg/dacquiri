# DACquiri
A compile-time enforced authorization framework for Rust applications.

## Authorization
In typical applications, authorization checks are performed in potentially random segments of the code. This leads to implicit assumptions on what kinds of permissions or checks have been enforced at various parts of the codebase. For example:

```rust
fn handler(req: Request) -> Result<Response, Error> {
  privileged_fn(req.get_user())
}

// makes no assumptions on a user's permissions or access, initially
fn privileged_fn(user: User) -> Result<Response, Error> {
  if !user.has(SimplePermission) { return Err(Error::PermissionError); }
  
  // action
  other_privileged_fn(user)
}

// Implicitly depends on user having the "SimplePermissions" permission or role. 
fn other_privileged_fn(user: User) -> Result<Response, Error> {
  if !user.has(AdvancedPermission) { return Err(Error::PermissionError); }
  
  // other action
  Ok(())
}
```

DACquiri does things differently.

With DACquiri, you explicitly declare your authorization requirements in the function definition. DACquiri will, at compile-time, enforce **all** code-paths invoking your function will have checked the appropriate authorization requirements beforehand.

With DACquiri, you:

* Know all of your authorization requirements based on your function's definition
* Know that all authorization requirements are enforced in all codepaths
* Know that authorization violations cannot be introduced accidentally

Missing an authorization check? That's a compile-time error.

Missing DACquiri? That's *your* error.

## How it works

DACquiri codifies permissions checks into the type system using a wrapper struct called `GrantChain`. For example, let's imagine you have two permissions called `P1` and `P2`. If you've checked both of these permissions on some `User` object, you might expect to now have a type `GrantChain<P2, GrantChain<P1, User>>`.

The magic of DACquiri is that it doesn't matter in which order you check permissions, just that you've checked them at some point. Regardless of the order, the outer `GrantChain` will implement both `HasGrant<P1>` as well as `HasGrant<P2>`. This is true for no matter how many grants you add to the chain.

Grants can be checked with the `try_grant` function where you'll specify which `Grant` you are currently checking. The actual check is performed in the `has_grant` function you must implement when implementing `Grant` on your `PrincipalT`. 

## Example

Here's a simplistic example of two permissions (`PermissionOne` and `PermissionTwo`) that we'll define as being grantable to all `User` objects (for the sake of this example).

```rust
use dacquiri::prelude::*;

impl_principal!(User);
struct User {
    name: String
}

struct PermissionOne;
struct PermissionTwo;

impl Grant for PermissionOne {
    type Principal = User;

    // give everyone this grant
    fn check_grant(_: &Self::Principal, _: &Self::Resource) -> Result<(), String> { Ok(()) }
    fn new_with_resource(_: Self::Resource) -> Self { Self }
    fn get_resource(&self) -> &Self::Resource { &() }
}

impl Grant for PermissionTwo {
    type Principal = User;

    // give everyone this grant
    fn check_grant(_: &Self::Principal, _: &Self::Resource) -> Result<(), String> { Ok(()) }
    fn new_with_resource(_: Self::Resource) -> Self { Self }
    fn get_resource(&self) -> &Self::Resource { &() }
}

fn requires_permission_one(caller: &impl HasGrant<PermissionOne>) {
    println!("The caller must have checked that you have PermissionOne");
}

fn requires_permission_two(caller: &impl HasGrant<PermissionTwo>) {
    println!("The caller must have checked that you have PermissionTwo");
}

fn requires_both_permission(
    caller: impl HasGrant<PermissionOne>
               + HasGrant<PermissionTwo>
) {
    println!("The caller must have checked that you had both PermissionOne and PermissionTwo");
}

fn main() -> Result<(), String> {
    let user = User { name: format!("d0nut") };
    
    let p1_user = user.try_grant::<PermissionOne, _>(())?;
    requires_permission_one(&p1_user);

    let p2_user = p1_user.try_grant::<PermissionTwo, _>(())?;
    requires_permission_two(&p2_user);

    requires_both_permission(p2_user);
}
```