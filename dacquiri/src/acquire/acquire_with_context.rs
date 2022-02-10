use async_trait::async_trait;
use crate::acquire::acquire_with_resource_and_context::AcquireAttributeWithResourceAndContext;
use crate::attribute::{
    AsyncAttribute,
    SyncAttribute
};
use crate::chain::{ConstraintChain, ConstraintEntity, EntityTag};
use crate::error::ConstraintError;

impl<T> AcquireAttributeWithContext for T
    where
        T: AcquireAttributeWithResourceAndContext<()>, {}

#[async_trait]
pub trait AcquireAttributeWithContext: AcquireAttributeWithResourceAndContext<()> {
    async fn constrain_with_resource_async<
        'ctx,
        Attr,
        const STAG: EntityTag,
        const RTAG: EntityTag,
    >(self) -> Result<ConstraintChain<STAG, RTAG, Attr, Self>, Attr::Error>
        where
            Attr::Subject: ConstraintEntity + 'static,
            Attr::Resource: ConstraintEntity + 'static,
            Attr: AsyncAttribute<Context<'ctx> = ()>,
    {
        let subject: &Attr::Subject = self.get_entity::<_, STAG>()
            .ok_or(ConstraintError::FailedToFetchSubject)?;

        let resource: &Attr::Resource = self.get_entity::<_, RTAG>()
            .ok_or(ConstraintError::FailedToFetchResource)?;

        Attr::test_async(subject, resource, ()).await?;

        Ok(ConstraintChain::<_, _, _, _>::new(self))
    }

    fn constrain_with_resource<
        'ctx,
        Attr,
        const STAG: EntityTag,
        const RTAG: EntityTag
    >(self) -> Result<ConstraintChain<STAG, RTAG, Attr, Self>, Attr::Error>
        where
            Attr::Subject: ConstraintEntity + 'static,
            Attr::Resource: ConstraintEntity + 'static,
            Attr: SyncAttribute<Context<'ctx> = ()>,
    {
        let subject: &Attr::Subject = self.get_entity::<_, STAG>()
            .ok_or(ConstraintError::FailedToFetchSubject)?;

        let resource: &Attr::Resource = self.get_entity::<_, RTAG>()
            .ok_or(ConstraintError::FailedToFetchResource)?;

        Attr::test(subject, resource, ())?;

        Ok(ConstraintChain::<_, _, _, _>::new(self))
    }
}
