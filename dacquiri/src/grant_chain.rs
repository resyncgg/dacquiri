use std::marker::PhantomData;
use crate::grants::{BaseGrant, GrantableWithResourceAndContext, HasGrant};
use crate::principal::PrincipalT;
use crate::resource::ResourceT;

auto trait UnequalTuple {}
impl<T> !UnequalTuple for (T, T) {}

#[derive(Clone)]
pub struct GrantChain<const ID: &'static str, P, R, G, N>
    where
        P: PrincipalT<P>,
        R: ResourceT,
        G: BaseGrant<ID, Principal = P, Resource = R>,
        N: PrincipalT<P>
{
    principal: PhantomData<P>,
    grant_hold: G,
    next: N
}

impl<const ID: &'static str, P, R, G, N> GrantChain<ID, P, R, G, N>
    where
        P: PrincipalT<P>,
        R: ResourceT,
        G: BaseGrant<ID, Principal = P, Resource = R>,
        N: PrincipalT<P>
{
    pub fn new(next: N, resource: R) -> Self {
        GrantChain::<ID, P, R, G, N> {
            principal: PhantomData,
            grant_hold: G::new_with_resource(resource),
            next
        }
    }
}

impl<P, R1, C1, R2, C2, G, N, const ID: &'static str> GrantableWithResourceAndContext<P, R1, C1> for GrantChain<ID, P, R2, G, N>
    where
        P: PrincipalT<P>,
        R1: ResourceT,
        R2: ResourceT,
        C1: Send,
        C2: Send,
        G: BaseGrant<ID, Principal = P, Resource = R2, Context<'static> = C2>,
        N: GrantableWithResourceAndContext<P, R1, C1>
{}

impl<const ID: &'static str, P, R, G, N> HasGrant<G, ID> for GrantChain<ID, P, R, G, N>
    where
        P: PrincipalT<P>,
        R: ResourceT,
        G: BaseGrant<ID, Principal = P, Resource = R>,
        N: PrincipalT<P>
{
    fn get_resource(&self) -> &G::Resource {
        self.grant_hold.get_resource()
    }
}

impl<const ID: &'static str, P, R, G, N> PrincipalT<P> for GrantChain<ID, P, R, G, N>
    where
        P: PrincipalT<P>,
        R: ResourceT,
        G: BaseGrant<ID, Principal = P, Resource = R>,
        N: PrincipalT<P>
{
    fn into_principal(self) -> P { self.next.into_principal() }
    fn get_principal(&self) -> &P { self.next.get_principal() }
    fn get_principal_mut(&mut self) -> &mut P {
        self.next.get_principal_mut()
    }
}

impl<const ID1: &'static str, const ID2: &'static str, P, R1, R2, G1, G2, CG> HasGrant<G2, ID2>
    for GrantChain<ID1, P, R1, G1, CG>
    where
        P: PrincipalT<P>,
        R1: ResourceT,
        R2: ResourceT,
        G1: BaseGrant<ID1, Principal = P, Resource = R1>,
        G2: BaseGrant<ID2, Principal = P, Resource = R2>,
        CG: HasGrant<G2, ID2>,
        (G1, G2): UnequalTuple
{
    fn get_resource(&self) -> &G2::Resource {
        self.next.get_resource()
    }
}