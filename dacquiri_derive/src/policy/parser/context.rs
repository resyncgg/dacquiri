use std::collections::HashSet;
use syn::{Token, TypeParamBound};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use crate::policy::entity_set::{EntityRef, EntitySet};
use super::clauses::Clause;

/// A branch in the policy where, if all conditions are met, the caller is considered authorized
pub struct Context {
    clauses: Vec<Clause>,
}

impl Context {
    pub(crate) fn generate_context_trait_bound(&self) -> Punctuated<TypeParamBound, Token![+]> {
        let mut trait_bound: Punctuated<TypeParamBound, Token![+]> = Punctuated::new();

        for clause in &self.clauses {
            trait_bound.push(clause.generate_clause_trait_bound());
        }

        trait_bound
    }
}

impl Parse for Context {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        syn::parenthesized!(content in input);

        let punctuated_clauses: Punctuated<Clause, Token![,]> = content.parse_terminated(Clause::parse)?;
        let clauses = punctuated_clauses.into_iter().collect();

        Ok(Self {
            clauses
        })
    }
}

impl EntitySet for Context {
    fn common_entities(&self) -> HashSet<EntityRef> {
        self.clauses.iter()
            .map(|clause| clause.common_entities())
            .flatten()
            .collect()
    }
}