use std::collections::HashSet;
use syn::parse::{Parse, ParseStream};
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