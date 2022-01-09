use std::marker::PhantomData;
use crate::grant::{Grant, Grantable, HasGrant};
use crate::principal::PrincipalT;


auto trait UnequalTuple {}
impl<T> !UnequalTuple for (T, T) {}

pub struct GrantChain<const ID: &'static str, P: PrincipalT<P>, R, G: Grant<ID, Principal = P, Resource = R>, N: PrincipalT<P>> {
    principal: PhantomData<P>,
    grant_hold: G,
    next: N
}

impl<const ID: &'static str, P: PrincipalT<P>, R, G: Grant<ID, Principal = P, Resource = R>, N: PrincipalT<P>> GrantChain<ID, P, R, G, N> {
    pub fn new(next: N, resource: R) -> Self {
        GrantChain::<ID, P, R, G, N> {
            principal: PhantomData,
            grant_hold: G::new_with_resource(resource),
            next
        }
    }
}

impl<P, R1, R2, G, N, const ID: &'static str> Grantable<P, R1> for GrantChain<ID, P, R2, G, N>
    where
        P: PrincipalT<P>,
        G: Grant<ID, Principal = P, Resource = R2>,
        N: Grantable<P, R1>
{}

impl<const ID: &'static str, P: PrincipalT<P>, R, G: Grant<ID, Principal = P, Resource = R>, N: PrincipalT<P>> HasGrant<G, ID> for GrantChain<ID, P, R, G, N>  {
    fn get_resource(&self) -> &G::Resource {
        self.grant_hold.get_resource()
    }
}

impl<const ID: &'static str, P: PrincipalT<P>, R, G: Grant<ID, Principal = P, Resource = R>, N: PrincipalT<P>> PrincipalT<P> for GrantChain<ID, P, R, G, N> {
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
> HasGrant<G2, ID2> for GrantChain<ID1, P, R1, G1, CG> where (G1, G2): UnequalTuple {
    fn get_resource(&self) -> &G2::Resource {
        self.next.get_resource()
    }
}