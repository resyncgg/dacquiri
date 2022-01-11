use crate::grant_chain::GrantChain;
use crate::grants::{BaseGrant, GrantableWithResourceAndContext};
use crate::principal::PrincipalT;
use crate::DEFAULT_GRANT_TAG;
use async_trait::async_trait;
use crate::grants::grant::{AsyncGrant, SyncGrant};

impl<P, G> Grantable<P> for G
    where
        P: PrincipalT<P>,
        G: GrantableWithResourceAndContext<P, (), ()> {}

#[async_trait]
pub trait Grantable<P>: GrantableWithResourceAndContext<P, (), ()>
    where
        P: PrincipalT<P>
{
    async fn try_grant_async<'ctx, G>(self) -> Result<GrantChain<DEFAULT_GRANT_TAG, P, (), G, Self>, G::Error>
        where
            G: AsyncGrant<DEFAULT_GRANT_TAG, Principal = P, Resource = (), Context<'ctx> = ()>,
    {
        self.try_grant_with_resource_and_context_async((), ()).await
    }

    fn try_grant<'ctx, G>(self) -> Result<GrantChain<DEFAULT_GRANT_TAG, P, (), G, Self>, G::Error>
        where
            G: SyncGrant<DEFAULT_GRANT_TAG, Principal = P, Resource = (), Context<'ctx> = ()>,
    {
        self.try_grant_with_resource_and_context((), ())
    }

}