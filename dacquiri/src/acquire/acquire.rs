use async_trait::async_trait;
use crate::acquire::acquire_with_resource_and_context::AcquireAttributeWithResourceAndContext;
use crate::attribute::{
    AsyncAttribute,
    SyncAttribute
};
use crate::chain::{ConstraintChain, ConstraintResult, ConstraintElement, ElementTag};
use crate::error::ConstraintError;
use crate::DEFAULT_ELEMENT_TAG;

impl<T> AcquireAttribute for T
    where
        T: AcquireAttributeWithResourceAndContext<()>, {}

#[async_trait]
pub trait AcquireAttribute: AcquireAttributeWithResourceAndContext<()> {
    async fn constrain_async<
        'ctx,
        Attr,
        const STAG: ElementTag,
    >(self) -> Result<ConstraintChain<STAG, DEFAULT_ELEMENT_TAG, Attr, Self>, Attr::Error>
        where
            Attr::Subject: ConstraintElement + 'static,
            Attr: AsyncAttribute<Resource = (), Context<'ctx> = ()>,
    {
        let subject: &Attr::Subject = self.get_element::<_, STAG>()
            .ok_or(ConstraintError::FailedToFetchSubject)?;

        Attr::test_async(subject, &(), ()).await?;

        Ok(ConstraintChain::<_, _, _, _>::new(self))
    }

    fn constrain<
        'ctx,
        Attr,
        const STAG: ElementTag,
    >(self) -> Result<ConstraintChain<STAG, DEFAULT_ELEMENT_TAG, Attr, Self>, Attr::Error>
        where
            Attr::Subject: ConstraintElement + 'static,
            Attr: SyncAttribute<Resource = (), Context<'ctx> = ()>,
    {
        let subject: &Attr::Subject = self.get_element::<_, STAG>()
            .ok_or(ConstraintError::FailedToFetchSubject)?;

        Attr::test(subject, &(), ())?;

        Ok(ConstraintChain::<_, _, _, _>::new(self))
    }
}
