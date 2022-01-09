mod builder;

use syn::{AttributeArgs, ItemTrait, parse_macro_input};
use proc_macro::TokenStream;
use quote::ToTokens;
use crate::requirement::builder::RequirementBuilder;

pub(crate) fn handle_requirement(args: TokenStream, input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as ItemTrait);
    let attribute_args = parse_macro_input!(args as AttributeArgs);

    let mut builder = RequirementBuilder::try_from((attribute_args, derive_input))
        .expect("Invalid use of the requirement macro.");

    builder.process();

    builder.to_token_stream().into()
}