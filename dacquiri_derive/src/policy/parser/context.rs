use syn::Token;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use super::clauses::Clause;


/// A branch in the policy where, if all conditions are met, the caller is considered authorized
pub struct Context {
    clauses: Vec<Clause>,
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
