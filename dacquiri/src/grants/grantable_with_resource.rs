use crate::grant_chain::GrantChain;
use crate::grants::{BaseGrant, GrantableWithResourceAndContext};
use crate::principal::PrincipalT;
use crate::resource::ResourceT;
use async_trait::async_trait;
use crate::grants::grant::{AsyncGrant, SyncGrant};

impl<P, R, G> GrantableWithResource<P, R> for G
    where
        P: PrincipalT<P>,
        R: ResourceT,
        G: GrantableWithResourceAndContext<P, R, ()> {}
#[async_trait]
pub trait GrantableWithResource<P, R>: GrantableWithResourceAndContext<P, R, ()>
    where
        P: PrincipalT<P>,
        R: ResourceT
{
    async fn try_grant_with_resource_async<'ctx, G, const ID: &'static str>(self, resource: R) -> Result<GrantChain<ID, P, R, G, Self>, G::Error>
        where
            G: AsyncGrant<ID, Principal = P, Resource = R, Context<'ctx> = ()>,
            R: 'async_trait
    {
        self.try_grant_with_resource_and_context_async(resource, ()).await
    }

    fn try_grant_with_resource<'ctx, G, const ID: &'static str>(self, resource: R) -> Result<GrantChain<ID, P, R, G, Self>, G::Error>
        where
            G: SyncGrant<ID, Principal = P, Resource = R, Context<'ctx> = ()>,
    {
        self.try_grant_with_resource_and_context(resource, ())
    }
}