use std::marker::PhantomData;
use crate::attributes::{BaseAttribute, AttributeWithResourceAndContext, HasAttribute};
use crate::subject::SubjectT;
use crate::resource::ResourceT;

auto trait UnequalTuple {}
impl<T> !UnequalTuple for (T, T) {}

pub trait AttributeChainT {
    // TODO: try to remove R when https://github.com/rust-lang/rust/issues/87486 is fixed
    fn get_resource<G, R, const ID: &'static str>(&self) -> &G::Resource
        where
            G: BaseAttribute<ID, Resource = R>,
            Self: HasAttribute<G, ID>;
}

impl<P, R, G, N, const AID: &'static str> AttributeChainT for AttributeChain<AID, P, R, G, N>
    where
        P: SubjectT<P>,
        R: ResourceT,
        G: BaseAttribute<AID, Subject = P, Resource = R>,
        N: SubjectT<P>
{
    fn get_resource<GN, RN, const ID: &'static str>(&self) -> &GN::Resource
        where
            GN: BaseAttribute<ID, Resource = RN>,
            Self: HasAttribute<GN, ID>,
    {
        <Self as HasAttribute<GN, ID>>::_get_attribute_resource(self)
    }
}

#[derive(Clone)]
pub struct AttributeChain<const ID: &'static str, P, R, G, N>
    where
        P: SubjectT<P>,
        R: ResourceT,
        G: BaseAttribute<ID, Subject = P, Resource = R>,
        N: SubjectT<P>
{
    principal: PhantomData<P>,
    grant_hold: G,
    next: N
}

impl<const ID: &'static str, P, R, G, N> AttributeChain<ID, P, R, G, N>
    where
        P: SubjectT<P>,
        R: ResourceT,
        G: BaseAttribute<ID, Subject = P, Resource = R>,
        N: SubjectT<P>
{
    pub fn new(next: N, resource: R) -> Self {
        AttributeChain::<ID, P, R, G, N> {
            principal: PhantomData,
            grant_hold: G::new_with_resource(resource),
            next
        }
    }
}

impl<P, R1, C1, R2, C2, G, N, const ID: &'static str> AttributeWithResourceAndContext<P, R1, C1> for AttributeChain<ID, P, R2, G, N>
    where
        P: SubjectT<P>,
        R1: ResourceT,
        R2: ResourceT,
        C1: Send,
        C2: Send,
        G: BaseAttribute<ID, Subject = P, Resource = R2, Context<'static> = C2>,
        N: AttributeWithResourceAndContext<P, R1, C1>
{}

impl<const ID: &'static str, P, R, G, N> HasAttribute<G, ID> for AttributeChain<ID, P, R, G, N>
    where
        P: SubjectT<P>,
        R: ResourceT,
        G: BaseAttribute<ID, Subject = P, Resource = R>,
        N: SubjectT<P>
{
    fn _get_attribute_resource(&self) -> &G::Resource {
        self.grant_hold.get_resource()
    }
}

impl<const ID: &'static str, P, R, G, N> SubjectT<P> for AttributeChain<ID, P, R, G, N>
    where
        P: SubjectT<P>,
        R: ResourceT,
        G: BaseAttribute<ID, Subject = P, Resource = R>,
        N: SubjectT<P>
{
    fn into_subject(self) -> P { self.next.into_subject() }
    fn get_subject(&self) -> &P { self.next.get_subject() }
    fn get_subject_mut(&mut self) -> &mut P {
        self.next.get_subject_mut()
    }
}

impl<const ID1: &'static str, const ID2: &'static str, P, R1, R2, G1, G2, CG> HasAttribute<G2, ID2>
    for AttributeChain<ID1, P, R1, G1, CG>
    where
        P: SubjectT<P>,
        R1: ResourceT,
        R2: ResourceT,
        G1: BaseAttribute<ID1, Subject = P, Resource = R1>,
        G2: BaseAttribute<ID2, Subject = P, Resource = R2>,
        CG: HasAttribute<G2, ID2>,
        (G1, G2): UnequalTuple
{
    fn _get_attribute_resource(&self) -> &G2::Resource {
        self.next._get_attribute_resource()
    }
}