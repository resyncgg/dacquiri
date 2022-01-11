use crate::grant_chain::GrantChain;
use crate::DEFAULT_GRANT_TAG;
use crate::principal::PrincipalT;

pub trait Grant<const ID: &'static str = DEFAULT_GRANT_TAG> {
    type Principal: PrincipalT<Self::Principal>;
    type Resource = ();
    type Error = ();
    type Context = ();

    fn new_with_resource(resource: Self::Resource) -> Self;

    fn get_resource(&self) -> &Self::Resource;

    fn check_grant(principal: &Self::Principal, resource: &Self::Resource, context: Self::Context) -> Result<(), Self::Error>;
}

pub trait HasGrant<T: Grant<ID>, const ID: &'static str = DEFAULT_GRANT_TAG>: PrincipalT<T::Principal> {
    fn get_resource(&self) -> &T::Resource;
}

impl<P: PrincipalT<P>, R, C> GrantableWithResourceAndContext<P, R, C> for P {}
pub trait GrantableWithResourceAndContext<P: PrincipalT<P>, R, C>: Sized + PrincipalT<P> {
    fn try_grant_with_resource_and_context<G, const ID: &'static str>(self, resource: R, context: C) -> Result<GrantChain<ID, P, R, G, Self>, G::Error>
        where
            G: Grant<ID, Principal = P, Resource = R, Context = C>,
    {
        let principal = self.get_principal();

        G::check_grant(principal, &resource, context)?;

        Ok(GrantChain::<ID, P, R, G, Self>::new(self, resource))
    }
}

impl<P: PrincipalT<P>, R, G: GrantableWithResourceAndContext<P, R, ()>> GrantableWithResource<P, R> for G {}
pub trait GrantableWithResource<P: PrincipalT<P>, R>: GrantableWithResourceAndContext<P, R, ()> {
    fn try_grant_with_resource<G, const ID: &'static str>(self, resource: R) -> Result<GrantChain<ID, P, R, G, Self>, G::Error>
        where
            G: Grant<ID, Principal = P, Resource = R, Context = ()>,
    {
        self.try_grant_with_resource_and_context(resource, ())
    }
}

impl<P: PrincipalT<P>, C, G: GrantableWithResourceAndContext<P, (), C>> GrantableWithContext<P, C> for G {}
pub trait GrantableWithContext<P: PrincipalT<P>, C>: GrantableWithResourceAndContext<P, (), C> {
    fn try_grant_with_context<G>(self, context: C) -> Result<GrantChain<DEFAULT_GRANT_TAG, P, (), G, Self>, G::Error>
        where
            G: Grant<DEFAULT_GRANT_TAG, Principal = P, Resource = (), Context = C>,
    {
        self.try_grant_with_resource_and_context((), context)
    }
}

impl<P: PrincipalT<P>, G: GrantableWithResourceAndContext<P, (), ()>> Grantable<P> for G {}
pub trait Grantable<P: PrincipalT<P>>: GrantableWithResourceAndContext<P, (), ()> {
    fn try_grant<G>(self) -> Result<GrantChain<DEFAULT_GRANT_TAG, P, (), G, Self>, G::Error>
        where
            G: Grant<DEFAULT_GRANT_TAG, Principal = P, Resource = (), Context = ()>,
    {
        self.try_grant_with_resource_and_context((), ())
    }
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