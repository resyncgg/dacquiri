use crate::chain::{ConstraintT, EntityTag};
use crate::prelude::ConstraintStore;

pub trait InitializeConstraint: Sized + Send + Sync + 'static {
    fn begin_constraint<const STAG: EntityTag>(self) -> ConstraintStore {
        let mut store = ConstraintStore::new();

        store = store
            .add_entity::<STAG>(self)
            .expect("Can't be an error");

        store
    }
}

impl<T> InitializeConstraint for T
    where
        T: Send + Sync + Sized + 'static
{}