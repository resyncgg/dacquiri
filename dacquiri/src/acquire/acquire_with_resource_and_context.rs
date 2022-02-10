use async_trait::async_trait;
use crate::attribute::{
    AsyncAttribute,
    SyncAttribute
};
use crate::chain::{ConstraintChain, ConstraintEntity, EntityTag, ConstraintT};
use crate::error::ConstraintError;


impl<T, C> AcquireAttributeWithResourceAndContext<C> for T
    where
        T: ConstraintT + Sized,
        C: Send {}

#[async_trait]
pub trait AcquireAttributeWithResourceAndContext<C: Send>: Sized + ConstraintT {
    async fn constrain_with_resource_and_context_async<
        'ctx,
        Attr,
        const STAG: EntityTag,
        const RTAG: EntityTag,
    >(self, context: C) -> Result<ConstraintChain<STAG, RTAG, Attr, Self>, Attr::Error>
        where
            Attr::Subject: ConstraintEntity + 'static,
            Attr::Resource: ConstraintEntity + 'static,
            C: 'async_trait,
            Attr: AsyncAttribute<Context<'ctx> = C>,
    {
        let subject: &Attr::Subject = self.get_entity::<_, STAG>()
            .ok_or(ConstraintError::FailedToFetchSubject)?;

        let resource: &Attr::Resource = self.get_entity::<_, RTAG>()
            .ok_or(ConstraintError::FailedToFetchResource)?;

        Attr::test_async(subject, resource, context).await?;

        Ok(ConstraintChain::<_, _, _, _>::new(self))
    }

    fn constrain_with_resource_and_context<
        'ctx,
        Attr,
        const STAG: EntityTag,
        const RTAG: EntityTag
    >(self, context: C) -> Result<ConstraintChain<STAG, RTAG, Attr, Self>, Attr::Error>
        where
            Attr::Subject: ConstraintEntity + 'static,
            Attr::Resource: ConstraintEntity + 'static,
            Attr: SyncAttribute<Context<'ctx> = C>,
    {
        let subject: &Attr::Subject = self.get_entity::<_, STAG>()
            .ok_or(ConstraintError::FailedToFetchSubject)?;

        let resource: &Attr::Resource = self.get_entity::<_, RTAG>()
            .ok_or(ConstraintError::FailedToFetchResource)?;

        Attr::test(subject, resource, context)?;

        Ok(ConstraintChain::<_, _, _, _>::new(self))
    }
}
