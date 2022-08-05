use std::collections::HashSet;

/// Refers to the usage of an entity in a context, clause, or other policy construct
#[derive(Clone, Hash, Eq, PartialEq)]
pub(crate) struct EntityRef(String);

impl From<String> for EntityRef {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl ToString for EntityRef {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

pub(crate) trait EntitySet {
    fn common_entities(&self) -> HashSet<EntityRef>;
}