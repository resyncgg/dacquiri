use crate::grant_chain::GrantChain;
use crate::grants::{BaseGrant, GrantableWithResourceAndContext, SyncGrant};
use crate::principal::PrincipalT;
use async_trait::async_trait;
use crate::DEFAULT_GRANT_TAG;
use crate::grants::grant::AsyncGrant;

impl<P, C, G> GrantableWithContext<P, C> for G
    where
        P: PrincipalT<P>,
        C: Send,
        G: GrantableWithResourceAndContext<P, (), C> {}
#[async_trait]
pub trait GrantableWithContext<P, C>: GrantableWithResourceAndContext<P, (), C>
    where
        P: PrincipalT<P>,
        C: Send,
{
    async fn try_grant_with_context_async<'ctx, G>(self, context: C) -> Result<GrantChain<DEFAULT_GRANT_TAG, P, (), G, Self>, G::Error>
        where
            G: AsyncGrant<DEFAULT_GRANT_TAG, Principal = P, Resource = (), Context<'ctx> = C>,
            C: 'async_trait
    {
        self.try_grant_with_resource_and_context_async((), context).await
    }

    fn try_grant_with_context<'ctx, G>(self, context: C) -> Result<GrantChain<DEFAULT_GRANT_TAG, P, (), G, Self>, G::Error>
        where
            G: SyncGrant<DEFAULT_GRANT_TAG, Principal = P, Resource = (), Context<'ctx> = C>,
    {
        self.try_grant_with_resource_and_context((), context)
    }
}