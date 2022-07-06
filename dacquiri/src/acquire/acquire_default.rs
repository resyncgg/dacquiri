use async_trait::async_trait;
use crate::acquire::acquire_with_resource_and_context::AcquireAttributeWithResourceAndContext;
use crate::attribute::{
    AsyncAttribute,
    SyncAttribute
};
use crate::chain::{ConstraintChain, ConstraintEntity, EntityTag};
use crate::DEFAULT_ELEMENT_TAG;
use crate::has::HasEntityWithType;

impl<T> AcquireAttribute for T
    where
        T: AcquireAttributeWithResourceAndContext<()>, {}

#[async_trait]
pub trait AcquireAttribute: AcquireAttributeWithResourceAndContext<()> {
    async fn prove_async<
        'ctx,
        Attr,
        const STAG: EntityTag,
    >(self) -> Result<ConstraintChain<STAG, DEFAULT_ELEMENT_TAG, Attr, Self>, Attr::Error>
        where
            Attr::Subject: ConstraintEntity + 'static,
            Self: HasEntityWithType<STAG, Attr::Subject>,
            Attr: AsyncAttribute<Resource = (), Context<'ctx> = ()>,
    {
        let subject = self.get_entity::<_, STAG>();

        Attr::test_async(subject, &(), ()).await?;

        Ok(ConstraintChain::<_, _, _, _>::new(self))
    }

    fn prove<
        'ctx,
        Attr,
        const STAG: EntityTag,
    >(self) -> Result<ConstraintChain<STAG, DEFAULT_ELEMENT_TAG, Attr, Self>, Attr::Error>
        where
            Attr::Subject: ConstraintEntity + 'static,
            Self: HasEntityWithType<STAG, Attr::Subject>,
            Attr: SyncAttribute<Resource = (), Context<'ctx> = ()>,
    {
        let subject = self.get_entity::<_, STAG>();

        Attr::test(subject, &(), ())?;

        Ok(ConstraintChain::<_, _, _, _>::new(self))
    }
}
