use proc_macro::TokenStream;
use quote::ToTokens;
use crate::attribute::builder::AttributeBuilder;
use syn::{AttributeArgs, ItemFn, parse_macro_input};

mod builder;

pub(crate) fn handle_attribute(args: TokenStream, input: TokenStream) -> TokenStream {
    let attribute_check_function = parse_macro_input!(input as ItemFn);
    let attribute_args = parse_macro_input!(args as AttributeArgs);

    AttributeBuilder::try_from((attribute_args, attribute_check_function))
        .expect("Unable to create AttributeBuilder")
        .to_token_stream()
        .into()
}
