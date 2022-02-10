use crate::attribute::BaseAttribute;
use crate::chain::{ConstraintChain, EntityTag};
use crate::DEFAULT_ELEMENT_TAG;

#[marker] pub trait HasConstraint<Attr, const STAG: EntityTag = DEFAULT_ELEMENT_TAG, const RTAG: EntityTag = DEFAULT_ELEMENT_TAG>
    where
        Attr: BaseAttribute {}

impl<
    N,
    Attr,
    const STAG: EntityTag,
    const RTAG: EntityTag,
> HasConstraint<Attr, STAG, RTAG> for ConstraintChain<STAG, RTAG, Attr, N>
    where
        Attr: BaseAttribute {}

impl<
    N,
    Attr1,
    Attr2,
    const STAG1: EntityTag,
    const STAG2: EntityTag,
    const RTAG1: EntityTag,
    const RTAG2: EntityTag,
> HasConstraint<Attr2, STAG2, RTAG2> for ConstraintChain<STAG1, RTAG1, Attr1, N>
    where
        Attr1: BaseAttribute,
        Attr2: BaseAttribute,
        N: HasConstraint<Attr2, STAG2, RTAG2> {}