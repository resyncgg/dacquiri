use syn::{
    Ident,
    Token,
};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;


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