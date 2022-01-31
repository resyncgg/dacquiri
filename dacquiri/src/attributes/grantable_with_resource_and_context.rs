use crate::attribute_chain::AttributeChain;
use crate::subject::SubjectT;
use crate::resource::ResourceT;
use async_trait::async_trait;
use crate::attributes::attribute::{AsyncGrant, SyncGrant};

impl<P, R, C> AttributeWithResourceAndContext<P, R, C> for P
    where
        P: SubjectT<P>,
        R: ResourceT,
        C: Send, {}

#[async_trait]
pub trait AttributeWithResourceAndContext<P, R, C>
    where
        P: SubjectT<P>,
        R: ResourceT,
        C: Send,
        Self: Sized + SubjectT<P>
{
    async fn try_grant_with_resource_and_context_async<'ctx, G, const ID: &'static str>(
        self,
        resource: R,
        context: C
    ) -> Result<AttributeChain<ID, P, R, G, Self>, G::Error>
        where
            G: AsyncGrant<ID, Subject = P, Resource = R, Context<'ctx> = C>,
            R: 'async_trait,
            C: 'async_trait
    {
        let subject = self.get_subject();

        G::grant_async(&subject, &resource, context).await?;

        Ok(AttributeChain::<ID, P, R, G, Self>::new(self, resource))
    }

    fn try_grant_with_resource_and_context<'ctx, G, const ID: &'static str>(
        self,
        resource: R,
        context: C
    ) -> Result<AttributeChain<ID, P, R, G, Self>, G::Error>
        where
            G: SyncGrant<ID, Subject = P, Resource = R, Context<'ctx> = C>,
    {
        let subject = self.get_subject();

        G::grant(&subject, &resource, context)?;

        Ok(AttributeChain::<ID, P, R, G, Self>::new(self, resource))
    }
}