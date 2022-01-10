pub use crate::grant::{
    Grant,
    Grantable,
    HasGrant
};
pub use crate::grant_chain::GrantChain;
pub use crate::principal::PrincipalT;

pub use crate::DEFAULT_GRANT_LABEL;
pub use crate::get_resource;

#[cfg(feature = "derive")]
pub use dacquiri_derive::{
    Principal,
    requirement,
    grant
};

pub type GrantResult<E> = Result<(), E>;