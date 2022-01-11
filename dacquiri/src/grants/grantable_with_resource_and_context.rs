use crate::grant_chain::GrantChain;
use crate::grants::BaseGrant;
use crate::principal::PrincipalT;
use crate::resource::ResourceT;
use async_trait::async_trait;
use crate::grants::grant::{AsyncGrant, SyncGrant};

impl<P, R, C> GrantableWithResourceAndContext<P, R, C> for P
    where
        P: PrincipalT<P>,
        R: ResourceT,
        C: Send, {}

#[async_trait]
pub trait GrantableWithResourceAndContext<P, R, C>
    where
        P: PrincipalT<P>,
        R: ResourceT,
        C: Send,
        Self: Sized + PrincipalT<P>
{
    async fn try_grant_with_resource_and_context_async<'ctx, G, const ID: &'static str>(
        self,
        resource: R,
        context: C
    ) -> Result<GrantChain<ID, P, R, G, Self>, G::Error>
        where
            G: AsyncGrant<ID, Principal = P, Resource = R, Context<'ctx> = C>,
            R: 'async_trait,
            C: 'async_trait
    {
        let principal = self.get_principal().clone();
        let cloned_resource = resource.clone();

        G::check_grant_async(principal, cloned_resource, context).await?;

        Ok(GrantChain::<ID, P, R, G, Self>::new(self, resource))
    }

    fn try_grant_with_resource_and_context<'ctx, G, const ID: &'static str>(
        self,
        resource: R,
        context: C
    ) -> Result<GrantChain<ID, P, R, G, Self>, G::Error>
        where
            G: SyncGrant<ID, Principal = P, Resource = R, Context<'ctx> = C>,
    {
        let principal = self.get_principal().clone();
        let cloned_resource = resource.clone();

        G::check_grant(principal, cloned_resource, context)?;

        Ok(GrantChain::<ID, P, R, G, Self>::new(self, resource))
    }
}