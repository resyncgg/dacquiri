use std::marker::PhantomData;
use crate::attribute::BaseAttribute;
use crate::error::ConstraintError;
use crate::has::{HasEntityWithType, ShedNext};
use crate::private::PrivateConstraintT;
use crate::store::EntityProof;

pub type EntityTag = &'static str;
pub type ConstraintResult<T> = Result<T, ConstraintError>;
pub trait ConstraintEntity = Send + Sync;


pub struct ConstraintChain<const STAG: EntityTag, const RTAG: EntityTag, Attr, Next> {
    next: Next,
    attr_phantom: PhantomData<Attr>
}

impl<const STAG: EntityTag, const RTAG: EntityTag, Attr, Next> ConstraintChain<STAG, RTAG, Attr, Next>
    where
        Attr: BaseAttribute,
        Next: ConstraintT
{
    pub(crate) fn new(next: Next) -> Self {
        Self {
            next,
            attr_phantom: PhantomData
        }
    }
}

pub trait ConstraintT: PrivateConstraintT + Sized {
    fn add_entity<T, const ETAG: EntityTag>(self, entity: T) -> ConstraintResult<EntityProof<ETAG, T, Self>>
        where
            T: ConstraintEntity + 'static;

    fn get_entity<T, const TAG: EntityTag>(&self) -> &T
        where
            T: ConstraintEntity + 'static,
            Self: HasEntityWithType<TAG, T>;

    fn get_entity_mut<T, const TAG: EntityTag>(&mut self) -> &mut T
        where
            T: ConstraintEntity + 'static,
            Self: HasEntityWithType<TAG, T>;

    fn try_get_entity<T, const TAG: EntityTag>(&self) -> ConstraintResult<&T>
        where
            T: ConstraintEntity + 'static;

    fn try_get_entity_mut<T, const TAG: EntityTag>(&mut self) -> ConstraintResult<&mut T>
        where
            T: ConstraintEntity + 'static;
}

impl<const STAG: EntityTag, const RTAG: EntityTag, Attr, Next> PrivateConstraintT for ConstraintChain<STAG, RTAG, Attr, Next>
    where
        Attr: BaseAttribute,
        Next: ConstraintT,
{
    fn _private_add_entity<T, const ETAG: EntityTag>(&mut self, entity: T) -> ConstraintResult<()>
        where
            T: ConstraintEntity + 'static
    {
        self.next._private_add_entity::<T, ETAG>(entity)
    }

    fn _private_get_entity_ref<T, const ETAG: EntityTag>(&self) -> &T
        where
            T: ConstraintEntity + 'static
    {
        self.next._private_get_entity_ref::<T, ETAG>()
    }

    fn _private_get_entity_mut<T, const ETAG: EntityTag>(&mut self) -> &mut T
        where
            T: ConstraintEntity + 'static
    {
        self.next._private_get_entity_mut::<T, ETAG>()
    }

    fn _private_get_entity<T, const ETAG: EntityTag>(&mut self) -> T
        where
            T: ConstraintEntity + 'static
    {
        self.next._private_get_entity::<T, ETAG>()
    }

    fn _private_try_get_entity_ref<T, const ETAG: EntityTag>(&self) -> ConstraintResult<&T>
        where
            T: ConstraintEntity + 'static
    {
        self.next._private_try_get_entity_ref::<T, ETAG>()
    }

    fn _private_try_get_entity_mut<T, const ETAG: EntityTag>(&mut self) -> ConstraintResult<&mut T>
        where
            T: ConstraintEntity + 'static
    {
        self.next._private_try_get_entity_mut::<T, ETAG>()
    }
}

impl<
    const STAG: EntityTag,
    const RTAG: EntityTag,
    Attr,
    Next
> ConstraintT for ConstraintChain<STAG, RTAG, Attr, Next>
    where
        Attr: BaseAttribute,
        Next: ConstraintT,
{
    fn add_entity<T, const ETAG: EntityTag>(mut self, entity: T) -> ConstraintResult<EntityProof<ETAG, T, Self>>
        where
            T: ConstraintEntity + 'static,
    {
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

impl<
    Next,
    ChainNext,
    EntityType,
    Attr,
    const STAG: EntityTag,
    const RTAG: EntityTag,
    const ETAG: EntityTag
> ShedNext<ETAG, EntityType, Next> for ConstraintChain<STAG, RTAG, Attr, ChainNext>
    where
        Attr: BaseAttribute,
        ChainNext: ShedNext<ETAG, EntityType, Next> + ConstraintT,
        EntityType: ConstraintEntity + 'static,
{
    fn shed(self) -> (EntityType, Next) {
        self.next.shed()
    }
}