# DACquiri
A compile-time enforced authorization framework for Rust applications.

## Authorization
In typical applications, authorization checks are performed in seemingly random locations. This leads to implicit assumptions on what kinds of permissions or checks have been enforced at various parts of the codebase. For example:

```rust
fn handler(req: Request) -> Result<Response, Error> {
  privileged_fn(req.get_user())
}

// makes no assumptions on a user's permissions or access, initially
fn privileged_fn(user: User) -> Result<Response, Error> {
  if !user.has(SimplePermission) {
    return Err(Error::PermissionError);
  }
  
  // action
  other_privileged_fn(user)
}

// Implicitly depends on user having the "SimplePermissions" permission or role. 
fn other_privileged_fn(user: User) -> Result<Response, Error> {
  if !user.has(AdvancedPermission) {
    return Err(Error::PermissionError);
  }
  
  // advanced action
  
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
