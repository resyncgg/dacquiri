pub use crate::attributes::{
    BaseAttribute,
    AsyncGrant,
    SyncGrant,
    AttributeWithResourceAndContext,
    GrantableWithResource,
    GrantableWithContext,
    Grantable,
    HasAttribute,
};
pub use crate::attribute_chain::{
    AttributeChain,
    AttributeChainT
};
/// The required trait bound for any type defined as a subject!
///
/// The proper way to get this trait bound is by marking your structs with `#[derive(Subject)]`.
pub use crate::subject::SubjectT;
/// The required trait bound for any type defined as a resource.
///
/// In short, it must be Send + Sync because we _potentially_ may send it across threads.
pub use crate::resource::ResourceT;

#[doc(hidden)]
pub use crate::DEFAULT_ATTRIBUTE_TAG;

#[cfg(feature = "derive")]
pub use dacquiri_derive::{
    Subject,
    entitlement,
    attribute
};

/// The required return type for [`attribute`](crate::prelude::attribute) marked functions.
pub type AttributeResult<E> = Result<(), E>;