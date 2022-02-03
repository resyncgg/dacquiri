use crate::chain::{ConstraintT, ElementTag};
use crate::prelude::ConstraintStore;

pub trait InitializeConstraint: Sized + Send + Sync + 'static {
    fn begin_constraint<const STAG: ElementTag>(self) -> ConstraintStore {
        let mut store = ConstraintStore::new();

        store = store
            .add_element::<STAG>(self)
            .expect("Can't be an error");

        store
    }
}

impl<T> InitializeConstraint for T
    where
        T: Send + Sync + Sized + 'static
{}