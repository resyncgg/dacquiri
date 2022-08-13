use std::collections::HashSet;
use syn::{
    Ident,
    Token,
};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use crate::policy::entity_set::{EntityRef, EntitySet};


pub(crate) struct DependentPolicy {
    pub policy_name: Ident,
    pub entities: Vec<Ident>
}

impl Parse for DependentPolicy {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let policy_name = input.parse()?;

        let content;
        syn::parenthesized!(content in input);

        let punctuated_entities: Punctuated<Ident, Token![,]> = content.parse_terminated(Ident::parse)?;
        let entities = punctuated_entities.into_iter().collect();

        Ok(Self {
            policy_name,
            entities
        })
    }
}

impl EntitySet for DependentPolicy {
    fn entities(&self) -> HashSet<EntityRef> {
        /*
            note:
                we can't know which of the passed in entities are actually used when validating
                the dependent policy without a specific analysis of that policy's definition.

                We'll rely on the implementer of the parent policy to specify the entities that should
                be required to ensure a specific context is satisfiable
         */
        HashSet::new()
    }
}