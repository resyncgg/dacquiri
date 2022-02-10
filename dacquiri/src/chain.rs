use std::any::Any;
use std::marker::PhantomData;
use fxhash::FxHashMap;
use crate::attribute::BaseAttribute;
use crate::error::ConstraintError;

pub type EntityTag = &'static str;
pub type ConstraintResult<T> = Result<T, ConstraintError>;
pub trait ConstraintEntity = Send + Sync;

pub struct ConstraintStore {
    entity_map: FxHashMap<&'static str, Box<dyn Any>>
}

impl ConstraintStore {
    pub(crate) fn new() -> Self {
        Self {
            entity_map: FxHashMap::default()
        }
    }
}

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


pub trait ConstraintT: Sized {
    fn add_entity<const TAG: EntityTag>(self, entity: impl ConstraintEntity + 'static) -> ConstraintResult<Self>;
    fn get_entity<T: ConstraintEntity + 'static, const TAG: EntityTag>(&self) -> Option<&T>;
}

impl ConstraintT for ConstraintStore {
    fn add_entity<const TAG: EntityTag>(mut self, entity: impl ConstraintEntity + 'static) -> ConstraintResult<Self> {
        // if we previously held an entity under this key and we attempted to overwrite it
        // we need to destroy the chain because it is invalid now
        match self.entity_map.insert(TAG, Box::new(entity)) {
            Some(_) => Err(ConstraintError::FailedToAddEntity),
            None => Ok(self)
        }
    }

    fn get_entity<T: ConstraintEntity + 'static, const TAG: EntityTag>(&self) -> Option<&T> {
        self.entity_map
            .get(&TAG)
            .and_then(|boxed| boxed.downcast_ref())
    }
}

impl<const STAG: EntityTag, const RTAG: EntityTag, Attr, Next> ConstraintT for ConstraintChain<STAG, RTAG, Attr, Next>
    where
        Attr: BaseAttribute,
        Next: ConstraintT
{
    fn add_entity<const TAG: EntityTag>(self, entity: impl ConstraintEntity + 'static) -> ConstraintResult<Self> {
        let next = self.next.add_entity::<TAG>(entity)?;

        Ok(ConstraintChain::<_, _, _, _>::new(next))
    }

    fn get_entity<T: ConstraintEntity + 'static, const TAG: EntityTag>(&self) -> Option<&T> {
        self.next.get_entity::<T, TAG>()
    }
}