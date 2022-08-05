mod policy;
mod entities;
mod context;
mod clauses;

pub(crate) use policy::Policy;
use crate::utils::NonstandardKeyword;

pub(crate) const ENTITIES_KEYWORD: &str = "entities";
pub(crate) const POLICIES_KEYWORD: &str = "policies";
pub(crate) const CONTEXT_KEYWORD: &str = "context";
pub(crate) const IS_KEYWORD: &str = "is";

pub(crate) type IsKeyword = NonstandardKeyword<IS_KEYWORD>;