use std::any::Any;
use std::marker::PhantomData;
use fxhash::FxHashMap;
use crate::chain::{ConstraintResult, ConstraintEntity, ConstraintT, EntityTag};
use crate::error::ConstraintError;
use crate::has::{HasEntityWithType, ShedNext};
use crate::private::PrivateConstraintT;

pub struct ConstraintStore {
    entity_map: FxHashMap<&'static str, Box<dyn Any + Send + Sync>>
}

impl ConstraintStore {
    pub(crate) fn new() -> Self {
        Self {
            entity_map: FxHashMap::default()
        }
    }
}

impl PrivateConstraintT for ConstraintStore {
    fn _private_add_entity<T, const ETAG: EntityTag>(&mut self, entity: T) -> ConstraintResult<()>
        where
            T: ConstraintEntity + 'static
    {
        match self.entity_map.insert(ETAG, Box::new(entity)) {
            Some(_) => Err(ConstraintError::EntityAlreadyExists),
            None => Ok(())
        }
    }

    fn _private_get_entity_ref<T, const ETAG: EntityTag>(&self) -> &T
        where
            T: ConstraintEntity + 'static
    {
        self.entity_map
            .get(&ETAG)
            .and_then(|boxed| boxed.downcast_ref())
            .expect("This should be impossible!!!")
    }

    fn _private_get_entity_mut<T, const ETAG: EntityTag>(&mut self) -> &mut T
        where
            T: ConstraintEntity + 'static
    {
        self.entity_map
            .get_mut(&ETAG)
            .and_then(|boxed| boxed.downcast_mut())
            .expect("This should be impossible!!!")
    }

    fn _private_get_entity<T, const ETAG: EntityTag>(&mut self) -> T
        where
            T: ConstraintEntity + 'static
    {
        *self.entity_map
            .remove(&ETAG)
            .and_then(|boxed| boxed.downcast().ok())
            .expect("This should be impossible!!!")
    }

    fn _private_try_get_entity_ref<T, const ETAG: EntityTag>(&self) -> ConstraintResult<&T> where T: ConstraintEntity + 'static {
        self.entity_map
            .get(&ETAG)
            .and_then(|boxed| boxed.downcast_ref())
            .ok_or(ConstraintError::EntityDoesNotExist)
    }

    fn _private_try_get_entity_mut<T, const ETAG: EntityTag>(&mut self) -> ConstraintResult<&mut T> where T: ConstraintEntity + 'static {
        self.entity_map
            .get_mut(&ETAG)
            .and_then(|boxed| boxed.downcast_mut())
            .ok_or(ConstraintError::EntityDoesNotExist)
    }
}

impl ConstraintT for ConstraintStore {
    fn add_entity<T, const ETAG: EntityTag>(mut self, entity: T) -> ConstraintResult<EntityProof<ETAG, T, Self>>
        where
            T: ConstraintEntity + 'static,
    {
        // if we previously held an entity under this key and we attempted to overwrite it
        // we need to destroy the chain because it is invalid now
        self._private_add_entity::<T, ETAG>(entity)
            .map(|_| EntityProof::<_, _, _>::new(self))
    }

    fn get_entity<T, const TAG: EntityTag>(&self) -> &T
        where
            T: ConstraintEntity + 'static,
            Self: HasEntityWithType<TAG, T>
    {
        self._private_get_entity_ref::<T, TAG>()
    }

    fn get_entity_mut<T, const TAG: EntityTag>(&mut self) -> &mut T
        where
            T: ConstraintEntity + 'static,
            Self: HasEntityWithType<TAG, T>
    {
        self._private_get_entity_mut::<T, TAG>()
    }

    fn try_get_entity<T, const TAG: EntityTag>(&self) -> ConstraintResult<&T>
        where
            T: ConstraintEntity + 'static
    {
        self._private_try_get_entity_ref::<T, TAG>()
    }

    fn try_get_entity_mut<T, const TAG: EntityTag>(&mut self) -> ConstraintResult<&mut T>
        where
            T: ConstraintEntity + 'static
    {
        self._private_try_get_entity_mut::<T, TAG>()
    }
}

pub struct EntityProof<const TAG: EntityTag, EntityType, Next> {
    next: Next,
    _entity_type: PhantomData<EntityType>,
}

impl<
    Next,
    EntityType,
    const ETAG: EntityTag
> ShedNext<ETAG, EntityType, Next> for EntityProof<ETAG, EntityType, Next>
    where
        Next: ConstraintT,
        EntityType: ConstraintEntity + 'static
{
    fn shed(mut self) -> (EntityType, Next) {
        let entity: EntityType = self._private_get_entity::<_, ETAG>();

        (entity, self.next)
    }
}

impl<
    Next,
    EntityType,
    const TAG: EntityTag
> EntityProof<TAG, EntityType, Next> {
    pub(crate) fn new(next: Next) -> Self {
        Self {
            next,
            _entity_type: PhantomData,
        }
    }
}

impl<
    Next,
    EntityType,
    const ETAG: EntityTag
> PrivateConstraintT for EntityProof<ETAG, EntityType, Next>
    where
        Next: ConstraintT
{
    fn _private_add_entity<T, const TAG: EntityTag>(&mut self, entity: T) -> ConstraintResult<()>
        where
            T: ConstraintEntity + 'static
    {
        self.next._private_add_entity::<T, TAG>(entity)
    }

    fn _private_get_entity_ref<T, const TAG: EntityTag>(&self) -> &T
        where
            T: ConstraintEntity + 'static
    {
        self.next._private_get_entity_ref::<T, TAG>()
    }

    fn _private_get_entity_mut<T, const TAG: EntityTag>(&mut self) -> &mut T
        where
            T: ConstraintEntity + 'static
    {
        self.next._private_get_entity_mut::<T, TAG>()
    }

    fn _private_get_entity<T, const TAG: EntityTag>(&mut self) -> T
        where
            T: ConstraintEntity + 'static
    {
        self.next._private_get_entity::<T, TAG>()
    }

    fn _private_try_get_entity_ref<T, const TAG: EntityTag>(&self) -> ConstraintResult<&T>
        where
            T: ConstraintEntity + 'static
    {
        self.next._private_try_get_entity_ref::<T, TAG>()
    }

    fn _private_try_get_entity_mut<T, const TAG: EntityTag>(&mut self) -> ConstraintResult<&mut T>
        where
            T: ConstraintEntity + 'static
    {
        self.next._private_try_get_entity_mut::<T, TAG>()
    }
}

impl<
    Next,
    EntityType,
    const ETAG: EntityTag
> ConstraintT for EntityProof<ETAG, EntityType, Next>
    where
        Next: ConstraintT
{
    fn add_entity<T, const TAG: EntityTag>(mut self, entity: T) -> ConstraintResult<EntityProof<TAG, T, Self>>
        where
            T: ConstraintEntity + 'static,
    {
        self._private_add_entity::<T, TAG>(entity)
            .map(|_| EntityProof::<_, _, _>::new(self))
    }

    fn get_entity<T, const TAG: EntityTag>(&self) -> &T
        where
            T: ConstraintEntity + 'static,
            Self: HasEntityWithType<TAG, T>
    {
        self._private_get_entity_ref::<T, TAG>()
    }

    fn get_entity_mut<T, const TAG: EntityTag>(&mut self) -> &mut T
        where
            T: ConstraintEntity + 'static,
            Self: HasEntityWithType<TAG, T>
    {
        self._private_get_entity_mut::<T, TAG>()
    }

    fn try_get_entity<T, const TAG: EntityTag>(&self) -> ConstraintResult<&T>
        where
            T: ConstraintEntity + 'static
    {
        self._private_try_get_entity_ref::<T, TAG>()
    }

    fn try_get_entity_mut<T, const TAG: EntityTag>(&mut self) -> ConstraintResult<&mut T>
        where
            T: ConstraintEntity + 'static
    {
        self._private_try_get_entity_mut::<T, TAG>()
    }
}