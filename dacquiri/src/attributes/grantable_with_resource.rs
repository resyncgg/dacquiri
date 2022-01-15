use crate::attribute_chain::AttributeChain;
use crate::attributes::AttributeWithResourceAndContext;
use crate::subject::SubjectT;
use crate::resource::ResourceT;
use async_trait::async_trait;
use crate::attributes::attribute::{AsyncGrant, SyncGrant};

impl<P, R, G> GrantableWithResource<P, R> for G
    where
        P: SubjectT<P>,
        R: ResourceT,
        G: AttributeWithResourceAndContext<P, R, ()> {}
#[async_trait]
pub trait GrantableWithResource<P, R>: AttributeWithResourceAndContext<P, R, ()>
    where
        P: SubjectT<P>,
        R: ResourceT
{
    async fn try_grant_with_resource_async<'ctx, G, const ID: &'static str>(self, resource: R) -> Result<AttributeChain<ID, P, R, G, Self>, G::Error>
        where
            G: AsyncGrant<ID, Subject= P, Resource = R, Context<'ctx> = ()>,
            R: 'async_trait
    {
        self.try_grant_with_resource_and_context_async(resource, ()).await
    }

    fn try_grant_with_resource<'ctx, G, const ID: &'static str>(self, resource: R) -> Result<AttributeChain<ID, P, R, G, Self>, G::Error>
        where
            G: SyncGrant<ID, Subject= P, Resource = R, Context<'ctx> = ()>,
    {
        self.try_grant_with_resource_and_context(resource, ())
    }
}