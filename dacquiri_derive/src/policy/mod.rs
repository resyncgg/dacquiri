mod builder;
mod parser;
mod entity_set;

use syn::{ItemTrait, parse_macro_input};
use proc_macro::TokenStream;
use quote::ToTokens;
use crate::policy::builder::PolicyBuilder;
use crate::policy::parser::Policy;

pub(crate) fn handle_policy(args: TokenStream, input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as ItemTrait);
    let policy = parse_macro_input!(args as Policy);

    let policy_builder: PolicyBuilder = (policy, derive_input)
        .try_into()
        .expect("Invalid input.");

    policy_builder
        .to_token_stream()
        .into()
}

