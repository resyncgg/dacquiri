use crate::grant_chain::GrantChain;
use crate::DEFAULT_GRANT_TAG;
use crate::principal::PrincipalT;
use crate::resource::ResourceT;
use async_trait::async_trait;

pub trait BaseGrant<const ID: &'static str = DEFAULT_GRANT_TAG>: Clone + Send {
    type Principal: PrincipalT<Self::Principal>;
    type Resource: ResourceT = ();
    type Error = ();
    type Context<'ctx> = ();

    fn new_with_resource(resource: Self::Resource) -> Self;

    fn get_resource(&self) -> &Self::Resource;
}

#[async_trait]
pub trait AsyncGrant<const ID: &'static str = DEFAULT_GRANT_TAG>: BaseGrant<ID> {
    async fn check_grant_async<'ctx>(principal: Self::Principal, resource: Self::Resource, context: Self::Context<'ctx>) -> Result<(), Self::Error>;
}

pub trait SyncGrant<const ID: &'static str = DEFAULT_GRANT_TAG>: BaseGrant<ID> {
    fn check_grant<'ctx>(principal: Self::Principal, resource: Self::Resource, context: Self::Context<'ctx>) -> Result<(), Self::Error>;
}

#[macro_export]
macro_rules! get_resource {
    ($from:ident as $ty:tt[$id:literal]) => {
        dacquiri::prelude::HasGrant::<$ty<{ $id }>, { $id }>::get_resource($from)
    };
    ($from:ident as $ty:ty) => {
        dacquiri::prelude::HasGrant::<$ty, { dacquiri::prelude::DEFAULT_GRANT_LABEL }>::get_resource($from)
    };
}