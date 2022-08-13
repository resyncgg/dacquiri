use std::collections::HashSet;
use proc_macro2::Span;
use syn::{ConstParam, Token, TypeParamBound};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Ident, parse_quote};
use crate::policy::entity_set::{EntityRef, EntitySet};
use super::clauses::Clause;

/// A branch in the policy where, if all conditions are met, the caller is considered authorized
pub struct Context {
    clauses: Vec<Clause>,
}

impl Context {
    pub(crate) fn clauses(&self) -> &Vec<Clause> {
        &self.clauses
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
    fn entities(&self) -> HashSet<EntityRef> {
        self.clauses.iter()
            .map(|clause| clause.entities())
            .flatten()
            .collect()
    }
}