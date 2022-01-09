use proc_macro::TokenStream;
use quote::ToTokens;
use crate::grant::builder::GrantBuilder;
use syn::{AttributeArgs, ItemFn, parse_macro_input};

mod builder;

pub(crate) fn handle_grant(args: TokenStream, input: TokenStream) -> TokenStream {
    let grant_check_function = parse_macro_input!(input as ItemFn);
    let grant_args = parse_macro_input!(args as AttributeArgs);

    GrantBuilder::try_from((grant_args, grant_check_function))
        .expect("Unable to create GrantBuilder")
        .to_token_stream()
        .into()
}
