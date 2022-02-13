use crate::chain::EntityTag;
use crate::prelude::ConstraintStore;
use crate::private::PrivateConstraintT;
use crate::store::EntityProof;

pub trait InitializeConstraint: Sized + Send + Sync + 'static {
    fn into_entity<const ETAG: EntityTag>(self) -> EntityProof<ETAG, Self, ConstraintStore> {
        let mut store = ConstraintStore::new();

        store
            ._private_add_entity::<Self, ETAG>(self)
            .expect("This should be impossible! No entities have been added yet.");

        EntityProof::<_, _, _>::new(store)
    }
}

impl<T> InitializeConstraint for T
    where
        T: Send + Sync + Sized + 'static
{}