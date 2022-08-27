use async_trait::async_trait;
use crate::acquire::acquire_with_resource_and_context::AcquireAttributeWithResourceAndContext;
use crate::attribute::{
    AsyncAttribute,
    SyncAttribute
};
use crate::chain::{ConstraintChain, ConstraintEntity, EntityTag};
use crate::DEFAULT_ELEMENT_TAG;
use crate::has::HasEntityWithType;


impl<T, C> AcquireAttributeWithContext<C> for T
    where
        T: AcquireAttributeWithResourceAndContext<C>,
        C: Send {}

#[async_trait]
pub trait AcquireAttributeWithContext<C: Send>: AcquireAttributeWithResourceAndContext<C> {
    async fn prove_with_context_async<
        'ctx,
        Attr,
        const STAG: EntityTag,
    >(self, context: C) -> Result<ConstraintChain<STAG, DEFAULT_ELEMENT_TAG, Attr, Self>, Attr::Error>
        where
            Attr::Subject: ConstraintEntity + 'static,
            C: 'async_trait,
            Self: HasEntityWithType<STAG, Attr::Subject>,
            Attr: AsyncAttribute<Resource = (), Context<'ctx> = C>,
    {
        let subject = self.get_entity::<_, STAG>();

        Attr::test_async(subject, &(), context).await?;

        Ok(ConstraintChain::<_, _, _, _>::new(self))
    }

    fn prove_with_context<
        'ctx,
        Attr,
        const STAG: EntityTag,
    >(self, context: C) -> Result<ConstraintChain<STAG, DEFAULT_ELEMENT_TAG, Attr, Self>, Attr::Error>
        where
            Attr::Subject: ConstraintEntity + 'static,
            Self: HasEntityWithType<STAG, Attr::Subject>,
            Attr: SyncAttribute<Resource = (), Context<'ctx> = C>,
    {
        let subject = self.get_entity::<_, STAG>();

        Attr::test(subject, &(), context)?;

        Ok(ConstraintChain::<_, _, _, _>::new(self))
    }
}
