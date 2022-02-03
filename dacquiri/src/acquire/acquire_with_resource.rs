use async_trait::async_trait;
use crate::acquire::acquire_with_resource_and_context::AcquireAttributeWithResourceAndContext;
use crate::attribute::{
    AsyncAttribute,
    SyncAttribute
};
use crate::chain::{ConstraintChain, ConstraintResult, ConstraintElement, ElementTag};
use crate::error::ConstraintError;
use crate::DEFAULT_ELEMENT_TAG;


impl<T, C> AcquireAttributeWithResource<C> for T
    where
        T: AcquireAttributeWithResourceAndContext<C>,
        C: Send {}

#[async_trait]
pub trait AcquireAttributeWithResource<C: Send>: AcquireAttributeWithResourceAndContext<C> {
    async fn constrain_with_context_async<
        'ctx,
        Attr,
        const STAG: ElementTag,
    >(self, context: C) -> Result<ConstraintChain<STAG, DEFAULT_ELEMENT_TAG, Attr, Self>, Attr::Error>
        where
            Attr::Subject: ConstraintElement + 'static,
            C: 'async_trait,
            Attr: AsyncAttribute<Resource = (), Context<'ctx> = C>,
    {
        let subject: &Attr::Subject = self.get_element::<_, STAG>()
            .ok_or(ConstraintError::FailedToFetchSubject)?;

        Attr::test_async(subject, &(), context).await?;

        Ok(ConstraintChain::<_, _, _, _>::new(self))
    }

    fn constrain_with_context<
        'ctx,
        Attr,
        const STAG: ElementTag,
    >(self, context: C) -> Result<ConstraintChain<STAG, DEFAULT_ELEMENT_TAG, Attr, Self>, Attr::Error>
        where
            Attr::Subject: ConstraintElement + 'static,
            Attr: SyncAttribute<Resource = (), Context<'ctx> = C>,
    {
        let subject: &Attr::Subject = self.get_element::<_, STAG>()
            .ok_or(ConstraintError::FailedToFetchSubject)?;

        Attr::test(subject, &(), context)?;

        Ok(ConstraintChain::<_, _, _, _>::new(self))
    }
}
