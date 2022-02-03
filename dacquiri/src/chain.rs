use std::any::{Any, TypeId};
use std::marker::PhantomData;
use fxhash::FxHashMap;
use crate::attribute::BaseAttribute;
use crate::error::ConstraintError;

pub type ElementTag = &'static str;
pub type ConstraintResult<T> = Result<T, ConstraintError>;
pub trait ConstraintElement = Send + Sync;

pub struct ConstraintStore {
    element_map: FxHashMap<&'static str, Box<dyn Any>>
}

impl ConstraintStore {
    pub fn new() -> Self {
        Self {
            element_map: FxHashMap::default()
        }
    }
}

pub struct ConstraintChain<const STAG: ElementTag, const RTAG: ElementTag, Attr, Next> {
    next: Next,
    attr_phantom: PhantomData<Attr>
}

impl<const STAG: ElementTag, const RTAG: ElementTag, Attr, Next> ConstraintChain<STAG, RTAG, Attr, Next>
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
    fn add_element<const TAG: ElementTag>(self, element: impl ConstraintElement + 'static) -> ConstraintResult<Self>;
    fn get_element<T: ConstraintElement + 'static, const TAG: ElementTag>(&self) -> Option<&T>;
}

impl ConstraintT for ConstraintStore {
    fn add_element<const TAG: ElementTag>(mut self, element: impl ConstraintElement + 'static) -> ConstraintResult<Self> {
        // if we previously held an element under this key and we attempted to overwrite it
        // we need to destroy the chain because it is invalid now
        match self.element_map.insert(TAG, Box::new(element)) {
            Some(_) => Err(ConstraintError::FailedToAddElement),
            None => Ok(self)
        }
    }

    fn get_element<T: ConstraintElement + 'static, const TAG: ElementTag>(&self) -> Option<&T> {
        self.element_map
            .get(&TAG)
            .and_then(|boxed| boxed.downcast_ref())
    }
}

impl<const STAG: ElementTag, const RTAG: ElementTag, Attr, Next> ConstraintT for ConstraintChain<STAG, RTAG, Attr, Next>
    where
        Attr: BaseAttribute,
        Next: ConstraintT
{
    fn add_element<const TAG: ElementTag>(self, element: impl ConstraintElement + 'static) -> ConstraintResult<Self> {
        let next = self.next.add_element::<TAG>(element)?;

        Ok(ConstraintChain::<_, _, _, _>::new(next))
    }

    fn get_element<T: ConstraintElement + 'static, const TAG: ElementTag>(&self) -> Option<&T> {
        self.next.get_element::<T, TAG>()
    }
}