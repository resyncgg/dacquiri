mod policy;
mod entities;
pub(crate) mod branch;
pub(crate) mod clauses;

pub(crate) use policy::Policy;
pub(crate) use entities::EntityDeclaration;
use crate::utils::NonstandardKeyword;

pub(crate) const ENTITIES_KEYWORD: &str = "entities";
pub(crate) const BRANCH_KEYWORD: &str = "branch";
pub(crate) const IS_KEYWORD: &str = "is";

pub(crate) type IsKeyword = NonstandardKeyword<IS_KEYWORD>;