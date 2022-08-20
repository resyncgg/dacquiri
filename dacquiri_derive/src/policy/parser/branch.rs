use std::collections::HashSet;
use syn::Token;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use crate::policy::entity_set::{EntityRef, EntitySet};
use super::clauses::Clause;

/// A branch in the policy where, if all conditions are met, the caller is considered authorized
pub struct Branch {
    clauses: Vec<Clause>,
}

impl Branch {
    pub(crate) fn clauses(&self) -> &Vec<Clause> {
        &self.clauses
    }
}


impl Parse for Branch {
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

impl EntitySet for Branch {
    fn entities(&self) -> HashSet<EntityRef> {
        self.clauses.iter()
            .map(|clause| clause.entities())
            .flatten()
            .collect()
    }
}