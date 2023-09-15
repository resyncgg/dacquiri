use crate::attribute::BaseAttribute;
use crate::chain::{ConstraintChain, EntityTag};
use crate::DEFAULT_ELEMENT_TAG;
use crate::store::EntityProof;

#[marker] pub trait HasConstraint<Attr, const STAG: EntityTag = DEFAULT_ELEMENT_TAG, const RTAG: EntityTag = DEFAULT_ELEMENT_TAG>
    where
        Attr: BaseAttribute {}

#[marker] pub trait HasEntity<const TAG: EntityTag> {}

#[marker] pub trait HasEntityWithType<const TAG: EntityTag, EntityType> {}

pub trait ShedNext<const TAG: EntityTag, EntityType, Next> {
    fn shed(self) -> (EntityType, Next);
}

/// Prove ConstraintChain<STAG, RTAG, Attr, _> => HasConstraint<Attr, STAG, RTAG>
impl<
    Next,
    Attr,
    const STAG: EntityTag,
    const RTAG: EntityTag,
> HasConstraint<Attr, STAG, RTAG> for ConstraintChain<STAG, RTAG, Attr, Next>
    where
        Attr: BaseAttribute {}

/// Prove ConstraintChain<_, _, _, HasConstraint<Attr2, STAG2, RTAG2>> => HasConstraint<Attr2, STAG2, RTAG2>
impl<
    Next,
    Attr1,
    Attr2,
    const STAG1: EntityTag,
    const STAG2: EntityTag,
    const RTAG1: EntityTag,
    const RTAG2: EntityTag,
> HasConstraint<Attr2, STAG2, RTAG2> for ConstraintChain<STAG1, RTAG1, Attr1, Next>
    where
        Attr1: BaseAttribute,
        Attr2: BaseAttribute,
        Next: HasConstraint<Attr2, STAG2, RTAG2> {}

/// Prove ConstraintChain<_, _, _, HasEntity<ETAG>> => HasEntity<ETAG>
impl<
    Next,
    Attr,
    const STAG: EntityTag,
    const RTAG: EntityTag,
    const ETAG: EntityTag
> HasEntity<ETAG> for ConstraintChain<STAG, RTAG, Attr, Next>
    where
        Attr: BaseAttribute,
        Next: HasEntity<ETAG>
{}

/// Prove ConstraintChain<_, _, _, HasEntityWithType<ETAG, EntityType>> => HasEntityWithType<ETAG, EntityType>
impl<
    Next,
    Attr,
    EntityType,
    const STAG: EntityTag,
    const RTAG: EntityTag,
    const ETAG: EntityTag
> HasEntityWithType<ETAG, EntityType> for ConstraintChain<STAG, RTAG, Attr, Next>
    where
        Attr: BaseAttribute,
        Next: HasEntityWithType<ETAG, EntityType>
{}


/// Prove EntityProof<TAG, EntityType, _> => HasEntity<TAG>
impl<
    Next,
    EntityType,
    const TAG: EntityTag,
> HasEntity<TAG> for EntityProof<TAG, EntityType, Next> {}

/// Prove EntityProof<TAG, EntityType, _> => HasEntityWithType<TAG, EntityType>
impl<
    Next,
    EntityType,
    const TAG: EntityTag,
> HasEntityWithType<TAG, EntityType> for EntityProof<TAG, EntityType, Next> {}

/// Prove EntityProof<_, _, HasEntity<TAG2>> => HasEntity<TAG2>
impl<
    Next,
    EntityType,
    const TAG1: EntityTag,
    const TAG2: EntityTag,
> HasEntity<TAG2> for EntityProof<TAG1, EntityType, Next>
    where
        Next: HasEntity<TAG2> {}

/// Prove EntityProof<_, _, HasEntity<TAG2, EntityType2>> => HasEntity<TAG2, EntityType2>
impl<
    Next,
    EntityType1,
    EntityType2,
    const TAG1: EntityTag,
    const TAG2: EntityTag,
> HasEntityWithType<TAG2, EntityType2> for EntityProof<TAG1, EntityType1, Next>
    where
        Next: HasEntityWithType<TAG2, EntityType2> {}

/// Prove EntityProof<_, _, HasConstraint<Attr, STAG, RTAG>> => HasConstraint<Attr, STAG, RTAG>
impl<
    Next,
    EntityType,
    Attr,
    const ETAG: EntityTag,
    const STAG: EntityTag,
    const RTAG: EntityTag,
> HasConstraint<Attr, STAG, RTAG> for EntityProof<ETAG, EntityType, Next>
    where
        Attr: BaseAttribute,
        Next: HasConstraint<Attr, STAG, RTAG> {}
