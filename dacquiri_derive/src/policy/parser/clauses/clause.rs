use std::collections::{HashMap, HashSet};
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::Token;
use syn::TypeParamBound;
use syn::parse_quote;
use syn::Ident;
use crate::policy::entity_set::{EntityRef, EntitySet};
use super::{
    Constraint,
    DependentPolicy
};

pub(crate) enum Clause {
    Constraint(Constraint),
    Policy(DependentPolicy)
}

impl Parse for Clause {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // todo: probably need to peek this stream since parse likely consumes part of the stream
        if input.fork().parse::<Constraint>().is_ok() {
            input.parse::<Constraint>().map(|constraint| Clause::Constraint(constraint))
        } else {
            input.parse::<DependentPolicy>().map(|policy| Clause::Policy(policy))
        }
    }
}

impl EntitySet for Clause {
    fn entities(&self) -> HashSet<EntityRef> {
        match self {
            Clause::Constraint(constraint) => constraint.entities(),
            Clause::Policy(policy) => policy.entities()
        }
    }
}