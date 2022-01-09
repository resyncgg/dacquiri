use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, ItemStruct};
use crate::principal::builder::PrincipalBuilder;

mod builder;

pub(crate) fn handle_principal(input: TokenStream) -> TokenStream {
    // Todo: add generics
    let principal = parse_macro_input!(input as ItemStruct);

    PrincipalBuilder::from(principal.ident)
        .to_token_stream()
        .into()
}
