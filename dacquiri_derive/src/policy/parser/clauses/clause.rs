use std::collections::HashSet;
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

impl Clause {
    pub(crate) fn generate_clause_trait_bound(&self) -> TypeParamBound {
        match self {
            Clause::Constraint(Constraint { subject_id, attribute, resource_constraint, .. }) => {
                let resource = match resource_constraint {
                    Some(resource) => resource.resource_id.to_token_stream(),
                    None => parse_quote!{ { dacquiri::prelude::DEFAULT_ELEMENT_TAG } }
                };

                let output = parse_quote! {
                    dacquiri::prelude::HasConstraint<#attribute, #subject_id, #resource>
                };

                output
            },
            Clause::Policy(DependentPolicy { policy_name, entities }) => {
                let punctuated_bounds: Punctuated<Ident, Token![,]> = entities
                    .clone()
                    .into_iter()
                    .collect();

                let output = parse_quote! {
                    #policy_name<#punctuated_bounds>
                };

                output
            }
        }
    }
}

impl Parse for Clause {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // todo: probably need to peek this stream since parse likely consumes part of the stream
        if let Ok(constraint) = input.parse::<Constraint>() {
            return Ok(Clause::Constraint(constraint));
        }

        let policy = input.parse::<DependentPolicy>()?;
        Ok(Clause::Policy(policy))
    }
}

impl EntitySet for Clause {
    fn common_entities(&self) -> HashSet<EntityRef> {
        match self {
            Clause::Constraint(constraint) => constraint.common_entities(),
            Clause::Policy(policy) => policy.common_entities()
        }
    }
}