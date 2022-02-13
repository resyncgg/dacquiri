/// DO NOT INCLUDE IN PRELUDE

use crate::chain::{EntityTag, ConstraintEntity, ConstraintResult};

/// THIS SHOULD NOT BE PUBLIC
/// This is a sealed trait
pub trait PrivateConstraintT {
    fn _private_add_entity<T, const ETAG: EntityTag>(&mut self, entity: T) -> ConstraintResult<()>
        where
            T: ConstraintEntity + 'static;

    fn _private_get_entity_ref<T, const ETAG: EntityTag>(&self) -> &T
        where
            T: ConstraintEntity + 'static;

    fn _private_get_entity_mut<T, const ETAG: EntityTag>(&mut self) -> &mut T
        where
            T: ConstraintEntity + 'static;

    fn _private_get_entity<T, const ETAG: EntityTag>(&mut self) -> T
        where
            T: ConstraintEntity + 'static;
}