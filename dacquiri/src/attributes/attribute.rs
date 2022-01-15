use crate::DEFAULT_ATTRIBUTE_TAG;
use crate::subject::SubjectT;
use crate::resource::ResourceT;
use async_trait::async_trait;

pub trait BaseAttribute<const ID: &'static str = DEFAULT_ATTRIBUTE_TAG>: Send + Sync {
    type Subject: SubjectT<Self::Subject>;
    type Resource: ResourceT = ();
    type Error = ();
    type Context<'ctx> = ();

    fn new_with_resource(resource: Self::Resource) -> Self;

    fn get_resource(&self) -> &Self::Resource;
}

#[async_trait]
pub trait AsyncGrant<const ID: &'static str = DEFAULT_ATTRIBUTE_TAG>: BaseAttribute<ID> {
    async fn grant_async<'ctx>(subject: &Self::Subject, resource: &Self::Resource, context: Self::Context<'ctx>) -> Result<(), Self::Error>;
}

pub trait SyncGrant<const ID: &'static str = DEFAULT_ATTRIBUTE_TAG>: BaseAttribute<ID> {
    fn grant<'ctx>(subject: &Self::Subject, resource: &Self::Resource, context: Self::Context<'ctx>) -> Result<(), Self::Error>;
}