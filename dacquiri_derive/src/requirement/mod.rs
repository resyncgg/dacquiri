mod builder;

use syn::{ItemTrait, parse_macro_input, Token, Ident};
use proc_macro::TokenStream;
use proc_macro2::Literal;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use crate::requirement::builder::RequirementBuilder;

pub(crate) fn handle_requirement(args: TokenStream, input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as ItemTrait);
    let attribute_args = parse_macro_input!(args as RequirementBoundSet);

    let mut builder = RequirementBuilder::try_from((attribute_args, derive_input))
        .expect("Invalid use of the requirement macro.");

    builder.process();

    builder.to_token_stream().into()
}

pub(crate) struct RequirementBoundSet {
    bounds: Vec<RequirementBound>
}

impl Parse for RequirementBoundSet {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut bounds = Vec::new();

        while let Ok(bound) = input.parse() {
            bounds.push(bound);

            // process next comma
            let _ = input.parse::<Token![,]>();
        }

        Ok(RequirementBoundSet {
            bounds
        })
    }
}

pub(crate) struct RequirementBound {
    permission_ident: Ident,
    specifier: Option<RequirementBoundSpecifier>,
}

impl Parse for RequirementBound {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let permission_ident: Ident = input.parse()?;
        let specifier: Option<RequirementBoundSpecifier> = input.parse().ok();

        Ok(RequirementBound {
            permission_ident,
            specifier
        })
    }
}

pub(crate) struct RequirementBoundSpecifier {
    _as_token: Token![as],
    id_lit: Literal
}

impl Parse for RequirementBoundSpecifier {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _as_token: Token![as] = input.parse()?;
        let id_lit: Literal = input.parse()?;

        Ok(Self {
            _as_token,
            id_lit
        })
    }
}