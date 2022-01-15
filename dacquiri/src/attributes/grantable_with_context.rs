use crate::attribute_chain::AttributeChain;
use crate::attributes::{AttributeWithResourceAndContext, SyncGrant};
use crate::subject::SubjectT;
use async_trait::async_trait;
use crate::DEFAULT_ATTRIBUTE_TAG;
use crate::attributes::attribute::AsyncGrant;

impl<P, C, G> GrantableWithContext<P, C> for G
    where
        P: SubjectT<P>,
        C: Send,
        G: AttributeWithResourceAndContext<P, (), C> {}
#[async_trait]
pub trait GrantableWithContext<P, C>: AttributeWithResourceAndContext<P, (), C>
    where
        P: SubjectT<P>,
        C: Send,
{
    async fn try_grant_with_context_async<'ctx, G>(self, context: C) -> Result<AttributeChain<DEFAULT_ATTRIBUTE_TAG, P, (), G, Self>, G::Error>
        where
            G: AsyncGrant<DEFAULT_ATTRIBUTE_TAG, Subject= P, Resource = (), Context<'ctx> = C>,
            C: 'async_trait
    {
        self.try_grant_with_resource_and_context_async((), context).await
    }

    fn try_grant_with_context<'ctx, G>(self, context: C) -> Result<AttributeChain<DEFAULT_ATTRIBUTE_TAG, P, (), G, Self>, G::Error>
        where
            G: SyncGrant<DEFAULT_ATTRIBUTE_TAG, Subject= P, Resource = (), Context<'ctx> = C>,
    {
        self.try_grant_with_resource_and_context((), context)
    }
}