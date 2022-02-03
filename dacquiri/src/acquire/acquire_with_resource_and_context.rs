use async_trait::async_trait;
use crate::attribute::{
    AsyncAttribute,
    SyncAttribute
};
use crate::chain::{ConstraintChain, ConstraintResult, ConstraintElement, ElementTag, ConstraintT};
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
        const STAG: ElementTag,
        const RTAG: ElementTag,
    >(self, context: C) -> Result<ConstraintChain<STAG, RTAG, Attr, Self>, Attr::Error>
        where
            Attr::Subject: ConstraintElement + 'static,
            Attr::Resource: ConstraintElement + 'static,
            C: 'async_trait,
            Attr: AsyncAttribute<Context<'ctx> = C>,
    {
        let subject: &Attr::Subject = self.get_element::<_, STAG>()
            .ok_or(ConstraintError::FailedToFetchSubject)?;

        let resource: &Attr::Resource = self.get_element::<_, RTAG>()
            .ok_or(ConstraintError::FailedToFetchResource)?;

        Attr::test_async(subject, resource, context).await?;

        Ok(ConstraintChain::<_, _, _, _>::new(self))
    }

    fn constrain_with_resource_and_context<
        'ctx,
        Attr,
        const STAG: ElementTag,
        const RTAG: ElementTag
    >(self, context: C) -> Result<ConstraintChain<STAG, RTAG, Attr, Self>, Attr::Error>
        where
            Attr::Subject: ConstraintElement + 'static,
            Attr::Resource: ConstraintElement + 'static,
            Attr: SyncAttribute<Context<'ctx> = C>,
    {
        let subject: &Attr::Subject = self.get_element::<_, STAG>()
            .ok_or(ConstraintError::FailedToFetchSubject)?;

        let resource: &Attr::Resource = self.get_element::<_, RTAG>()
            .ok_or(ConstraintError::FailedToFetchResource)?;

        Attr::test(subject, resource, context)?;

        Ok(ConstraintChain::<_, _, _, _>::new(self))
    }
}
