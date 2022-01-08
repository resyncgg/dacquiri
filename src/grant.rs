use crate::grant_chain::GrantChain;
use crate::DEFAULT_GRANT_LABEL;
use crate::principal::PrincipalT;

pub trait Grant<const ID: &'static str = DEFAULT_GRANT_LABEL> {
    type Principal: PrincipalT<Self::Principal>;
    type Resource = ();
    type Error = ();

    fn new_with_resource(resource: Self::Resource) -> Self;

    fn get_resource(&self) -> &Self::Resource;

    fn check_grant(principal: &Self::Principal, resource: &Self::Resource) -> Result<(), Self::Error>;
}


pub trait HasGrant<T: Grant<ID>, const ID: &'static str = DEFAULT_GRANT_LABEL>: PrincipalT<T::Principal> {
    fn get_resource(&self) -> &T::Resource;
}

pub trait Grantable<P: PrincipalT<P>, R = ()>: Sized + PrincipalT<P> {
    fn try_grant<G, const ID: &'static str>(self, resource: R) -> Result<GrantChain<ID, P, R, G, Self>, G::Error>
        where
            G: Grant<ID, Principal = P, Resource = R>,
    {
        let principal = self.get_principal();

        G::check_grant(principal, &resource)?;

        Ok(GrantChain::<ID, P, R, G, Self>::new(self, resource))
    }
}

impl<P: PrincipalT<P>, R> Grantable<P, R> for P {}