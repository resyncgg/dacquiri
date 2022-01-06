use std::marker::PhantomData;

/// Is the label applied to grant checks that are not resource dependant.
pub const GRANT_CHECK_DEFAULT: &'static str = "GrantCheck::__";

pub auto trait UnequalTuple {}
impl<T> !UnequalTuple for (T, T) {}


pub trait Grant<const ID: &'static str = GRANT_CHECK_DEFAULT> {
    type Principal: PrincipalT<Self::Principal>;
    type Resource = ();
    const NAME: &'static str;

    fn name() -> &'static str { Self::NAME }

    fn new_with_resource(resource: Self::Resource) -> Self;

    fn get_resource(&self) -> &Self::Resource;

    fn check_grant(principal: &Self::Principal, resource: &Self::Resource) -> Result<(), String>;
}

pub trait PrincipalT<P = Self> {
    fn into_principal(self) -> P;
    fn get_principal(&self) -> &P;
    fn get_principal_mut(&mut self) -> &mut P;
}

pub struct GrantElement<const ID: &'static str, P: PrincipalT<P>, R, G: Grant<ID, Principal = P, Resource = R>, N: PrincipalT<P>> {
    principal: PhantomData<P>,
    grant_hold: G,
    next: N
}

impl<const ID: &'static str, P: PrincipalT<P>, R, G: Grant<ID, Principal = P, Resource = R>, N: PrincipalT<P>> GrantElement<ID, P, R, G, N> {
    pub fn new(next: N, resource: R) -> Self {
        GrantElement::<ID, P, R, G, N> {
            principal: PhantomData,
            grant_hold: G::new_with_resource(resource),
            next
        }
    }
}

pub trait HasGrant<T: Grant<ID>, const ID: &'static str = GRANT_CHECK_DEFAULT>: PrincipalT<T::Principal> {
    fn get_resource(&self) -> &T::Resource;
}

pub trait Grantable<P: PrincipalT<P>, R = ()>: Sized + PrincipalT<P> {
    fn try_grant<G, const ID: &'static str>(self, resource: R) -> Result<GrantElement<ID, P, R, G, Self>, String>
        where
            G: Grant<ID, Principal = P, Resource = R>,
    {
        let principal = self.get_principal();

        G::check_grant(principal, &resource)?;

        Ok(GrantElement::<ID, P, R, G, Self>::new(self, resource))
    }
}

impl<P, R1, R2, G, N, const ID: &'static str> Grantable<P, R1> for GrantElement<ID, P, R2, G, N>
    where
        P: PrincipalT<P>,
        G: Grant<ID, Principal = P, Resource = R2>,
        N: Grantable<P, R1>
{}

impl<const ID: &'static str, P: PrincipalT<P>, R, G: Grant<ID, Principal = P, Resource = R>, N: PrincipalT<P>> HasGrant<G, ID> for GrantElement<ID, P, R, G, N>  {
    fn get_resource(&self) -> &G::Resource {
        self.grant_hold.get_resource()
    }
}

impl<const ID: &'static str, P: PrincipalT<P>, R, G: Grant<ID, Principal = P, Resource = R>, N: PrincipalT<P>> PrincipalT<P> for GrantElement<ID, P, R, G, N> {
    fn into_principal(self) -> P { self.next.into_principal() }
    fn get_principal(&self) -> &P { self.next.get_principal() }
    fn get_principal_mut(&mut self) -> &mut P {
        self.next.get_principal_mut()
    }
}

impl<
    const ID1: &'static str,
    const ID2: &'static str,
    P: PrincipalT<P>,
    R1,
    R2,
    G1: Grant<ID1, Principal = P, Resource = R1>,
    G2: Grant<ID2, Principal = P, Resource = R2>,
    CG: HasGrant<G2, ID2>
> HasGrant<G2, ID2> for GrantElement<ID1, P, R1, G1, CG> where (G1, G2): UnequalTuple {
    fn get_resource(&self) -> &G2::Resource {
        self.next.get_resource()
    }
}

#[macro_export]
macro_rules! impl_principal {
    ($principal:ty) => {
        impl PrincipalT for $principal {
            fn into_principal(self) -> Self { self }
            fn get_principal(&self) -> &Self { self }
            fn get_principal_mut(&mut self) -> &mut Self { self }
        }
    }
}