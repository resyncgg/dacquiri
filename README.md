# Dacquiri
A framework that turns authorization vulnerabilities into compiler errors.

# What is Dacquiri?
Dacquiri is a framework that uses the type system to validate, at compile time, all code paths satisfy your access control policies. It does this by giving developers the capability to annotate any function with an access control policy. These policies are transformed into complex trait bounds that enforce that callers check that they satisfy these policies ahead of time.

# How Does It Work?
Dacquiri consists of two main components: **attributes** and **policies**.

## Attributes
*Attributes* are equivalent to the conditions you'd find in your `if` statements and other control flow logic.

Take the following example web endpoint in an Actix web application.

```rust
// An example endpoint built in actix.
// Assume that Session is an extractor
#[get("/documents/{doc_id}")]
async fn access_document(req: HttpRequest, session: Session, doc_id: Path<String>) -> impl Responder {
    let document_service = req.get_document_service();
    let doc_id = doc_id.into_inner();
    let document_meta = document_service.fetch_doc_metadata(doc_id).await?;

    // Only allow caller to read document if they own it
    if document_meta.owner == session.user_id {
        let document = document_service.fetch_doc_contents(doc_id).await?;

        Ok(document)
    } else {
        Err(AppError::Unauthorized)
    }
}
```

The `document.owner == session.user_id` condition is an example of an *attribute* you'd write in Dacquiri. 

Let's build that attribute with Dacquiri and see how we can use it to protect this application.

### Defining an Attribute

```rust
use dacquiri::prelude::*;

// define the attribute
#[attribute(Owner)]
mod owner {
    // define a method of testing for that attribute
    #[attribute]
    fn check_caller_owns_document(
        session: &Session,
        document_meta: &DocumentMeta
    ) -> AttributeResult<AppError> {
        // check user owns document
        (session.user_id == document_meta.owner)
            .then_some(())
            .ok_or(AppError::Unauthorized)
    }
}
```

Now we can use the `Owner` attribute to talk about whether or not we own a particular document. Now let's use it describe how we could safely fetch documents!

## Policies
*Policies* allow us to define the access control policy on a collection of methods. They're made up of **entities** and **guards**.

An *entity* is any object we want to test attributes against or access in our methods. 

A *guard* is a collection of attributes that must be satisfied to access this method.

Let's build a simple policy that only allows callers to fetch document contents if they own the document. We'll implement it as an async method on the policy trait definition. 

```rust
use dacquiri::prelude::*;

#[policy(
    entities = (
        user: Session,
        document_metadata: DocumentMeta
    ),
    guard = (
        user is Owner for document_metadata
    )
)]
pub trait DocumentOwnerPolicy {
    async fn fetch_document_contents(&self, document_service: &DocumentService) -> Result<Document, AppError> {
        // grab the DocumentMeta from our policy definition
        let meta: &DocumentMeta = self.get_entity::<_, document_metadata>();

        // fetch the document contents with the provided document_service
        let document = document_service.fetch_doc_contents(meta.doc_id).await?;

        // return the document!
        Ok(document)
    }
}
```

Policies are defined with a collection of constraints of the form:

```
<subject entity> is <attribute> [for <resource entity>]
```

Also, notice that we use `self.get_entity` to access the `document_meta` object defined in our policy definition.

Why is this important?

We want to make sure that the `DocumentMeta` object we use to fetch data is the exact same object that we used to validate the access control policy. 

This allows us to avoid the following kind of vulnerability:

```rust
let document_meta_one = document_service.fetch_doc_metadata(doc_id_one).await?;
let document_meta_two = document_service.fetch_doc_metadata(doc_id_two).await?;

// checking ownership of the first document...
if document_meta_one.owner == session.user_id {
    // Ahh! A vulnerability!
    // We're fetching the wrong document!
    // This uses `document_meta_two` instead of the tested `document_meta_one`
    let document = document_service.fetch_doc_contents(document_meta_two.doc_id).await?;

    Ok(document)
} else {
    Err(AppError::Unauthorized)
}
```

As long as an entity is defined in our `entities` section of the policy we can fetch it with `get_entity`. 

## Using Policies
Now that we have our document fetching method protected by a policy, how do call it?

First we need to coalesce the entities we want to prove together into an `EntityProof`. This manages the entities we've added and makes it easy to fetch entities in our policies.

When we add entities to our `EntityProof` we have to give them names. It's not sufficient to just rely on the type of the entity because we may need to talk about two or more entities of the same time. That's why it's important they each have distinct names. 

```rust
// coalesce our entities together
let entities = session
    .into_entity::<"user">()
    .add_entity::<_, "document_metadata">(document_meta)?;
```

Next, we check if attributes are true between our entities. We can do this by calling the attribute function, by name, that we defined earlier. For example, we defined the `Owner` attribute function as `check_caller_owns_document(...)` and can call it here.

```rust
// prove `Owner` for "user" and "document_metadata"
let proof = entities.check_caller_owns_document::<"user", "document_metadata">()?;
```

Now that we've added the check that proves `"user"` owns the document described by `"document_metadata"`, we can call our protected method!

Let's see this all together.

```rust
#[get("/documents/{doc_id}")]
async fn access_document(req: HttpRequest, session: Session, doc_id: Path<String>) -> impl Responder {
    let document_service = req.get_document_service();
    let doc_id = doc_id.into_inner();
    let document_meta = document_service.fetch_doc_metadata(doc_id).await?;

    // coalesce our entities
    let entities = session
        .into_entity::<"user">()
        .add_entity::<_, "document_metadata">(document_meta)?;

    // prove our properties
    let proof = entities.check_caller_owns_document::<"user", "document_metadata">()?;

    // call the protected function!
    proof.fetch_document_contents(&document_service).await
}
```

Of course you can chain all of these methods together if that makes things easier

# Advanced Attributes
Attributes aren't particularly complex (_partly as a feature_), but they do have some additional capabilities that may not be obvious.

## Subject, Resource, and Context
Attribute functions support up to three arguments.

The first argument is the **subject** entity. This entity must always be present and be an immutable reference to the entity type. 

```rust
#[attribute(Enabled)]
mod enabled {
    #[attribute]
    // 'User' is the subject entity type
    fn check_user_enabled(user: &User) -> AttributeResult<AppError> {
        // check user is enabled
        (user.enabled)
            .then_some(())
            .ok_or(AppError::Unauthorized)
    }
}
```

The second, optional, argument is the **resource** entity. There really isn't a meaningful different between the *subject* and *resource* except where they go in the policy constraint expression. Similar to *subject* entities, *resource* entities must also be an immutable reference to their entity type.

The final possible argument to an attribute function is the **context**. This is any object (_or collection of objects_) that help you verify an attribute. A canonical example of a *context* object is a database connection. Without this connection, you may not be able to query a database and validate some property is true. 

A context object _can_ be supplied without an associated resource and may or may not be a reference. If you wish to define an attribute function with only a *subject* entity and a *context* object, set the *resource* entity type to `&()` and it will be ignored. 

```rust
#[attribute(Adult)]
mod adult {
    #[attribute]
    fn check_user_is_adult(user: &User, _: &(), db: &DbConnection) -> AttributeResult<AppError> {
        const AGE_ADULT: u32 = 18;
        // use db to query user's current age
        // we'd *probably* expect this to be a property on `User`, but this is for the sake of the example
        let age = db.query_user_age(user.user_id)?;

        if age >= AGE_ADULT {
            Ok(())
        } else {
            Err(AppError::Unauthorized)
        }
    }
}
```

## Async Attributes
Attributes can be `async`! There's nothing special you need to do aside from writing the function to be `async`. This is especially useful with *context* objects like database connections or a grpc service. 

```rust
#[attribute(Member)]
mod member {
    #[attribute]
    async fn check_user_is_member_of_team(user: &User, team: &Team, service: &TeamService) -> AttributeResult<AppError> {
        // attempt to fetch the membership record of this user
        let membership: Option<Membership> = service.get_membership(user.user_id, team.team_id).await?;

        if membership.is_some() {
            Ok(())
        } else {
            Err(AppError::UserNotAMember)
        }
    }
}
```

When you go to test this attribute elsewhere in your code, it'll be an async method that you must `await` on as expected.

## Attribute Name Reuse
Attributes support defining multiple attribute functions allowing for different types of entities to prove a particular attribute. Attributes are still scoped to particular subject and resource entity types, preventing attribute confusion. 

The main benefit to allowing multiple attribute functions is that different entities can use the same attribute name to describe a relationship. For example, defining the following constraint is non-ideal from a readability perspective.

```rust
#[policy(
    entities = (
        user: User,
        team: Team,
    ),
    guard = (
        user is UserEnabled,
        team is TeamEnabled,
    )
)]
pub trait Something {}
```

By defining multiple attribute functions, we can reuse the attribute `Enabled` but have strong, type-checked proves for each entity type.

```rust
#[attribute(Enabled)]
mod enabled {
    #[attribute]
    fn check_user_is_enabled(user: &User) -> AttributeResult<AppError> {
        (user.enabled)
            .then_some(())
            .ok_or(AppError::UserNotEnabled)
    }

    #[attribute]
    fn check_team_is_enabled(team: &Team) -> AttributeResult<AppError> {
        (team.enabled)
            .then_some(())
            .ok_or(AppError::TeamNotEnabled)
    }
}

#[policy(
    entities = (
        user: User,
        team: Team,
    ),
    // this reads much better!
    guard = (
        user is Enabled,
        team is Enabled,
    )
)]
pub trait Something {}
```

# Advanced Policies
## Dependent Policies
In addition to using attributes, guards can depend on other policies by using the following syntax:

```
<policy_name>(<entities>)
```

For example, if we created a new policy that relied on our previous `DocumentOwnerPolicy`, we might define the guard in the following way:

```rust
#[policy(
    entities = (
        user: Session,
        document_metadata: DocumentMeta
    ),
    guard = (
        user is OtherAttribute,
        DocumentOwnerPolicy(user, document_metadata)
    )
)]
pub trait OtherPolicy {
    // prints document contents to stdout
    async fn do_stuff(&self, document_service: &DocumentService) -> Result<(), AppError> {
        // we can call `fetch_document_contents` because we're guaranteed to satisfy `DocumentOwnerPolicy`!
        let document = self.fetch_document_contents(document_service).await?;

        println!("Document contents: {}", document);
    }
}
```

Any methods inside of `OtherPolicy` would be able to call into methods defined by `DocumentOwnerPolicy`. This is true even if we don't explicitly depend on `DocumentOwnerPolicy` in our `guard` statement. As long as all of the policy constraints are known to be satisfied, our method can call methods guarded by other policies.

## Multiple Guards
Sometimes there are multiple contexts in which someone should be able to call into a given method. In our example so far, the caller must prove that a user owns a particular document before retriving its contents. But what if we had a background service that indexed documents for searching? How would that service fetch the document contents without a user session?

Policies support multiple guard conditions for such a case. Each guard condition is treated as a branch of an `OR` statement meaning that as long as one of the branches is satisfied, the caller can invoke the policy protected methods. 

Let's reinvision our `DocumentOwnerPolicy` to allow for a background service to access the document contents.

```rust 
#[policy(
    entities = (
        user: Session,
        service: ServiceSession,
        document_metadata: DocumentMeta
    ),
    guard = (
        user is Owner for document_metadata
    ),
    guard = (
        service is Valid
    )
)]
pub trait DocumentOwnerPolicy {
    async fn fetch_document_contents(&self, document_service: &DocumentService) -> Result<Document, AppError> {
        // grab the DocumentMeta from our policy definition
        let meta: &DocumentMeta = self.get_entity::<_, document_metadata>();

        // fetch the document contents with the provided document_service
        let document = document_service.fetch_doc_contents(meta.doc_id).await?;

        // return the document!
        Ok(document)
    }
}
```

Unfortunately, there are two major restrictions here.

The first is that if a policy uses multiple guards, no guard may use dependent policies. If it's important that your multi-guard policy be able to call into other policies, you can still require all of the attributes required but cannot depend on the policy itself.

The second restriction is that Dacquiri will require that all described entities are present for the policy to be satisified. That means that despite the fact that we only need a `ServiceSession` to be `Valid` to call into `fetch_document_contents`, we'll need to supply a user's `Session` regardless.

To avoid that problem Dacquiri supports *optional* entities!

## Optional Entities
Optional entities allow us to relax the requirement that described entities are present for a policy to be satisfied. To mark an entity as optional, add a `?` to the end of the type. 

Taking our previous example, we can mark the `ServiceSession` and `Session` types as optional!

```rust 
#[policy(
    // 'user' and 'service' are now optional types!
    entities = (
        user: Session?,
        service: ServiceSession?,
        document_metadata: DocumentMeta
    ),
    guard = (
        user is Owner for document_metadata
    ),
    guard = (
        service is Valid
    )
)]
pub trait DocumentOwnerPolicy {
    async fn fetch_document_contents(&self, document_service: &DocumentService) -> Result<Document, AppError> {
        // grab the DocumentMeta from our policy definition
        let meta: &DocumentMeta = self.get_entity::<_, document_metadata>();

        // fetch the document contents with the provided document_service
        let document = document_service.fetch_doc_contents(meta.doc_id).await?;

        // return the document!
        Ok(document)
    }
}
```

One important thing to note about optional entities is that any entity marked optional will not be able to be retrieved using `self.get_entity`, as this method uses compile-time checks to validate the entity is present. 

To access an optional entity, `self.try_get_entity` may be used. 
