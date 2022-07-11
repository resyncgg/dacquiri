# Dacquiri
An authorization framework with compile-time enforcement.

# Introduction to Dacquiri
Dacquiri turns authorization vulnerabilities into compile-time errors.

Dacquiri has two main concepts that govern how authorization policies are defined and applied.
### ❗ Note - Unstable Features
`dacquiri` relies on nightly + multiple unstable features to work.
The following unstable features will, at minimum, be required in your
application for it to work with `dacquiri`.
```rust
#![feature(generic_associated_types)]
#![feature(adt_const_params)]
#![feature(generic_arg_infer)]
```
Additionally, you can add `#![allow(incomplete_features)]` to ignore the inevitable unstable feature warnings.

# Attributes
**Attributes** are properties we prove about a **Subject**
(_the entity we are applying the authorization check against_).
_Attributes_ are statements that are true about a particular _subject_. For example,
`UserIsEnabled` may be an attribute defined for `User` _subjects_ that have their `enabled` flag set to `true`.
Some additional _attributes_ you might define could answer the following:
* Is this user's ID verified?
* Is this user's account older than 30 days?
* Is this user a member of a particular team?
  

  The last _attribute_ introduces us to the idea of _resources_.
  A **Resource** is the object a _subject_ is attempting to acquire a particular _attribute_ against.
  A common example of an _attribute_, with a _resource_, would be a `UserIsTeamMember` attribute.
  For this attribute, the *subject* is `User` and the *resource* is `Team`. This attribute would only be
  granted if the `User` was a member of the specified `Team`.
  

  While this a useful primitive, it wouldn't make much sense to check if a `User` was a member of a `Team` and then perform actions against a
  completely different `Team` object. Therefore, attributes _also_ **remember** which
  _resource_ they were acquired against. This way, if necessary, you can access an _attribute_'s associated _resource_.
## Writing Attributes
We define _attributes_ using the [`attribute`](crate::prelude::attribute) macro, 1 to 3 arguments, and an [`AttributeResult`](crate::prelude::AttributeResult) return type.
```rust
use dacquiri::prelude::*;
#[attribute(UserIsEnabled)]
fn check_user_is_enabled(user: &User) -> AttributeResult<String> {
    match user.enabled {
        true => Ok(()),
        false => Err(format!("User is not enabled."))
    }
}
```
This will automatically generate an _attribute_ with a `User` as the subject and `()` as the resource.
If we have a resource we depend on, we can add it as the second argument to the function.
```rust
use dacquiri::prelude::*;
#[attribute(UserIsTeamMember)]
fn check_user_team(
    user: &User,
    team: &Team
) -> AttributeResult<String> {
    match team.users.contains(&user.user_id) {
        true => Ok(()),
        false => Err(format!("User is not specified team."))
    }
}
```
The generated `UserIsTeamMember` attribute will have `User` as the _subject_ and `Team` as the _resource_.
Sometimes, you may not have all of the required information to determine if a _subject_ has a particular _attribute_
for a particular _resource_ even if you already have that _resource_ fetched. In these cases, you can specify an optional
third argument to provide _context_ or assets required to access additional, required information.

Here's an example iteration on the previous _attribute_ we defined where we fetch data, live, from a database.
```rust
use dacquiri::prelude::*;
#[attribute(UserIsTeamMember)]
async fn user_team_check(
    user: &User,
    team: &Team,
    conn: &mut DatabaseConnection
) -> AttributeResult<String> {
    let row_count = conn.count_query(
        "select count(*) from memberships where uid = {} and tid = {}",
        vec![user.user_id, team.team_id]
    )
    .await
    .map_err(|_| format!("DB error."))?;
    // if we have more than 1 records, we're on the team!
    match row_count > 0  {
        true => Ok(()),
        false => Err(format!("User is not on the specified team."))
    }
}
```
You should notice two things that are different about this particular attribute.
1. We didn't have to make the context (_3rd argument_) an immutable reference. Attribute context's
   can be owned, immutable, or mutable references. This allows you to use any concrete type you wish here.
2. You should also notice that this attribute function is `async`! Attributes support async and it's as
   simple as just adding the keyword to the function. All of the other work is handled automatically for you.
   We'll come back to attributes in a bit, but first let's talk about **Entitlements**.
# Entitlements
**Entitlements** are traits, gated behind one or more _attributes_, that are automatically applied
to any _subject_ that has acquired all of the prerequisite _attributes_ at _some point_, in _any order_.
An example of a useful _entitlement_ could be a `VerifiedUser` _entitlement_ which would require the following _attributes_:
* `UserIsEnabled` - Checks that the user's enabled flag is true
* `UserIsVerified` - Checks that the user's verified state is `Verified::Success`
## Writing Entitlements
Entitlements allow us to guard functionality behind a prerequisite set of _attributes_ using default trait methods.
We start by defining a trait with the [`entitlement`](crate::prelude::entitlement) macro.
```rust
#[entitlement(UserIsVerified, UserIsEnabled)]
pub trait VerifiedUser {
    fn print_message(&self) {
        println!("Hello, world!!");
    }
}
```
This _entitlement_ requires that a _subject_ have both the `UserIsVerified` and `UserIsEnabled` attributes.
If a _subject_ has acquired both _attributes_, `VerifiedUser` will automatically be implemented on the _subject_.
To get access to the `User` _subject_ again, we use the [`get_subject`](crate::prelude::SubjectT::get_subject) or
[`get_subject_mut`](crate::prelude::SubjectT::get_subject_mut) methods. Then we can access information
or make changes to our subject once again.
```rust
#[entitlement(UserIsVerified, UserIsEnabled)]
pub trait VerifiedUser {
    fn change_name(&mut self, new_name: impl Into<String>) {
        self.get_subject_mut().name = new_name.into();
    }
}
```
We can create async methods here as well using [`#[async_trait]`](async_trait::async_trait) like a normal trait.
```rust
#[async_trait]
#[entitlement(UserIsVerified, UserIsEnabled)]
pub trait VerifiedUser {
    // set the account's enabled to false and consume the user
    async fn disable_account(self, conn: &mut DatabaseConnection) {
        let query = escape!(
            "UPDATE users SET enabled = false WHERE uid = {};",
            self.get_subject().user_id
        );
        conn.execute(query).await;
    }
}
```
# Acquiring Attributes
To acquire an _attribute_, we call one of the following on our subject.
- [`try_grant`](crate::prelude::Grantable::try_grant)
- [`try_grant_async`](crate::prelude::Grantable::try_grant_async)
- [`try_grant_with_context`](crate::prelude::GrantableWithContext::try_grant_with_context)
- [`try_grant_with_context_async`](crate::prelude::GrantableWithContext::try_grant_with_context_async)
- [`try_grant_with_resource`](crate::prelude::GrantableWithResource::try_grant_with_resource)
- [`try_grant_with_resource_async`](crate::prelude::GrantableWithResource::try_grant_with_resource_async)
- [`try_grant_with_resource_and_context`](crate::prelude::AttributeWithResourceAndContext::try_grant_with_resource_and_context)
- [`try_grant_with_resource_and_context_async`](crate::prelude::AttributeWithResourceAndContext::try_grant_with_resource_and_context_async)
  
For example, if we wanted to check if our `User` was both enabled and a member of a `Team` we could do the following.
  We'll use the previous `UserIsEnabled` and `UserIsTeamMember` _attribute_ definitions.
```rust
#[tokio::main]
async fn main() -> Result<(), String> {
    let user: User = get_user();
    let team: Team = get_team();
    let mut conn: DatabaseConnection = get_database_conn();
    let checked_user = user
        .try_grant::<UserIsEnabled>()?
        .try_grant_with_resource_and_context_async::<UserIsTeamMember, _>(team, &mut conn).await?;
}
```
## Leveraging Entitlements
Now that we know how to acquire an _attribute_ for a _subject_, let's put the entitlement system
to work by guarding a function with one or more entitlements.

We treat _entitlements_ like regular traits and guard with your favorite trait-bound syntax.
Here's a longer, more complicated example, that demonstrates the value that `dacquiri` provides
by guarding access to the `leave_team` functionality to `Users` until they have checked both
_attributes_ required by the `TeamMember` _entitlement_ bound.

It does not matter the order that the `try_grant_*` functions are called, that they are called
sequentially, or that they even happened in the same function.
```rust
#[tokio::main]
async fn main() -> Result<(), String> {
    let user: User = get_user();
    let team: Team = get_team();
    let mut conn: DatabaseConnection = get_database_conn();
    let mut checked_user = user
        .try_grant::<UserIsEnabled>()?
        .try_grant_with_resource_and_context_async::<UserIsTeamMember, _>(team, &mut conn).await?;
    leave_my_team(&mut checked_user).await
}
async fn leave_my_team(user: impl TeamMember) -> Result<(), String> {
    // you can't call `.leave_team()` if you're not
    // a TeamMember (which requires UserIsEnabled and UserIsTeamMember)
    user.leave_team().await
}
#[entitlement(UserIsEnabled, UserIsTeamMember)]
#[async_trait]
trait TeamMember {
    // we capture self here because leaving the team
    // means we're no longer a team member
    async fn leave_team(
        self,
        conn: &mut DatabaseConection
) -> Result<(), String> {
        let user = self.get_subject();
        // we need to specify *which* attribute's resource we want
        let team = self.get_resource::<UserIsTeamMember, _, _>();
        let query = escape!(
            "DELETE FROM members WHERE uid = {} AND tid = {};",
            user.user_id,
            team.team_id
        );
        conn
            .execute(query)
            .await
            .map(|_| format!("DB error"))?;
        Ok(())
    }
}
```
# Subjects
The last topic that needs to be covered is about _subjects_. We mentioned them earlier; _subjects_ are the
entities that we're administering an authorization policy against and applying access control.
We do need to denote subjects before we can start acquiring attributes on them.
Do mark a struct as a `Subject` we mark them with `#[derive(Subject))`
```rust
use dacquiri::prelude::Subject;
#[derive(Subject)]
pub struct AuthenticatedUser {
    username: String,
    session_token: String,
    enabled: bool
}
```
That's it!

Now you have a relatively good grasp on how `dacquiri` works and how you can use it
to life authorization requirements into the type system.
