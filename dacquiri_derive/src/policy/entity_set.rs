use std::collections::HashSet;

/// Refers to the usage of an entity in a context, clause, or other policy construct
#[derive(Clone, Hash, Eq, PartialEq)]
pub(crate) struct EntityRef(String);

impl From<String> for EntityRef {
    fn from(value: String) -> Self {
        Self(value)
    }
}

pub(crate) trait EntitySet {
    fn common_entities(&self) -> HashSet<EntityRef>;
}