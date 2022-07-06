use async_trait::async_trait;
use crate::attribute::{
    AsyncAttribute,
    SyncAttribute
};
use crate::chain::{ConstraintChain, ConstraintEntity, EntityTag, ConstraintT};
use crate::has::HasEntityWithType;


impl<T, C> AcquireAttributeWithResourceAndContext<C> for T
    where
        T: ConstraintT + Sized,
        C: Send {}

#[async_trait]
pub trait AcquireAttributeWithResourceAndContext<C: Send>: Sized + ConstraintT {
    async fn prove_with_resource_and_context_async<
        'ctx,
        Attr,
        const STAG: EntityTag,
        const RTAG: EntityTag,
    >(self, context: C) -> Result<ConstraintChain<STAG, RTAG, Attr, Self>, Attr::Error>
        where
            Attr::Subject: ConstraintEntity + 'static,
            Attr::Resource: ConstraintEntity + 'static,
            C: 'async_trait,
            Self: HasEntityWithType<STAG, Attr::Subject>,
            Self: HasEntityWithType<RTAG, Attr::Resource>,
            Attr: AsyncAttribute<Context<'ctx> = C>,
    {
        let subject = self.get_entity::<_, STAG>();
        let resource = self.get_entity::<_, RTAG>();

        Attr::test_async(subject, resource, context).await?;

        Ok(ConstraintChain::<_, _, _, _>::new(self))
    }

    fn prove_with_resource_and_context<
        'ctx,
        Attr,
        const STAG: EntityTag,
        const RTAG: EntityTag
    >(self, context: C) -> Result<ConstraintChain<STAG, RTAG, Attr, Self>, Attr::Error>
        where
            Attr::Subject: ConstraintEntity + 'static,
            Attr::Resource: ConstraintEntity + 'static,
            Self: HasEntityWithType<STAG, Attr::Subject>,
            Self: HasEntityWithType<RTAG, Attr::Resource>,
            Attr: SyncAttribute<Context<'ctx> = C>,
    {
        let subject = self.get_entity::<_, STAG>();
        let resource = self.get_entity::<_, RTAG>();

        Attr::test(subject, resource, context)?;

        Ok(ConstraintChain::<_, _, _, _>::new(self))
    }
}
