use crate::attribute::BaseAttribute;
use crate::chain::{ConstraintChain, ElementTag};
use crate::DEFAULT_ELEMENT_TAG;

#[marker] pub trait HasConstraint<Attr, const STAG: ElementTag = DEFAULT_ELEMENT_TAG, const RTAG: ElementTag = DEFAULT_ELEMENT_TAG>
    where
        Attr: BaseAttribute {}

impl<
    N,
    Attr,
    const STAG: ElementTag,
    const RTAG: ElementTag,
> HasConstraint<Attr, STAG, RTAG> for ConstraintChain<STAG, RTAG, Attr, N>
    where
        Attr: BaseAttribute {}

impl<
    N,
    Attr1,
    Attr2,
    const STAG1: ElementTag,
    const STAG2: ElementTag,
    const RTAG1: ElementTag,
    const RTAG2: ElementTag,
> HasConstraint<Attr2, STAG2, RTAG2> for ConstraintChain<STAG1, RTAG1, Attr1, N>
    where
        Attr1: BaseAttribute,
        Attr2: BaseAttribute,
        N: HasConstraint<Attr2, STAG2, RTAG2> {}