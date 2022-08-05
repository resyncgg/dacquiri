mod policy;
mod entities;
pub(crate) mod context;
mod clauses;

pub(crate) use policy::Policy;
pub(crate) use entities::EntityDeclaration;
use crate::utils::NonstandardKeyword;

pub(crate) const ENTITIES_KEYWORD: &str = "entities";
pub(crate) const CONTEXT_KEYWORD: &str = "context";
pub(crate) const IS_KEYWORD: &str = "is";

pub(crate) type IsKeyword = NonstandardKeyword<IS_KEYWORD>;