use crate::attribute_chain::AttributeChain;
use crate::attributes::AttributeWithResourceAndContext;
use crate::subject::SubjectT;
use crate::DEFAULT_ATTRIBUTE_TAG;
use async_trait::async_trait;
use crate::attributes::attribute::{AsyncGrant, SyncGrant};

impl<P, G> Grantable<P> for G
    where
        P: SubjectT<P>,
        G: AttributeWithResourceAndContext<P, (), ()> {}

#[async_trait]
pub trait Grantable<P>: AttributeWithResourceAndContext<P, (), ()>
    where
        P: SubjectT<P>
{
    async fn try_grant_async<'ctx, G>(self) -> Result<AttributeChain<DEFAULT_ATTRIBUTE_TAG, P, (), G, Self>, G::Error>
        where
            G: AsyncGrant<DEFAULT_ATTRIBUTE_TAG, Subject = P, Resource = (), Context<'ctx> = ()>,
    {
        self.try_grant_with_resource_and_context_async((), ()).await
    }

    fn try_grant<'ctx, G>(self) -> Result<AttributeChain<DEFAULT_ATTRIBUTE_TAG, P, (), G, Self>, G::Error>
        where
            G: SyncGrant<DEFAULT_ATTRIBUTE_TAG, Subject = P, Resource = (), Context<'ctx> = ()>,
    {
        self.try_grant_with_resource_and_context((), ())
    }

}