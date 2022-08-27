use async_trait::async_trait;
use crate::acquire::acquire_with_resource_and_context::AcquireAttributeWithResourceAndContext;
use crate::attribute::{
    AsyncAttribute,
    SyncAttribute
};
use crate::chain::{ConstraintChain, ConstraintEntity, EntityTag};
use crate::has::HasEntityWithType;

impl<T> AcquireAttributeWithResource for T
    where
        T: AcquireAttributeWithResourceAndContext<()>, {}

#[async_trait]
pub trait AcquireAttributeWithResource: AcquireAttributeWithResourceAndContext<()> {
    async fn prove_with_resource_async<
        'ctx,
        Attr,
        const STAG: EntityTag,
        const RTAG: EntityTag,
    >(self) -> Result<ConstraintChain<STAG, RTAG, Attr, Self>, Attr::Error>
        where
            Attr::Subject: ConstraintEntity + 'static,
            Attr::Resource: ConstraintEntity + 'static,
            Self: HasEntityWithType<STAG, Attr::Subject>,
            Self: HasEntityWithType<RTAG, Attr::Resource>,
            Attr: AsyncAttribute<Context<'ctx> = ()>,
    {
        let subject = self.get_entity::<_, STAG>();
        let resource = self.get_entity::<_, RTAG>();

        Attr::test_async(subject, resource, ()).await?;

        Ok(ConstraintChain::<_, _, _, _>::new(self))
    }

    fn prove_with_resource<
        'ctx,
        Attr,
        const STAG: EntityTag,
        const RTAG: EntityTag
    >(self) -> Result<ConstraintChain<STAG, RTAG, Attr, Self>, Attr::Error>
        where
            Attr::Subject: ConstraintEntity + 'static,
            Attr::Resource: ConstraintEntity + 'static,
            Self: HasEntityWithType<STAG, Attr::Subject>,
            Self: HasEntityWithType<RTAG, Attr::Resource>,
            Attr: SyncAttribute<Context<'ctx> = ()>,
    {
        let subject = self.get_entity::<_, STAG>();
        let resource = self.get_entity::<_, RTAG>();

        Attr::test(subject, resource, ())?;

        Ok(ConstraintChain::<_, _, _, _>::new(self))
    }
}
