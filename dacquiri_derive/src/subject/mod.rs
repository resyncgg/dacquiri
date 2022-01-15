use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, ItemStruct};
use crate::subject::builder::SubjectBuilder;

mod builder;

pub(crate) fn handle_subject(input: TokenStream) -> TokenStream {
    // Todo: add generics
    let subject = parse_macro_input!(input as ItemStruct);

    SubjectBuilder::from(subject)
        .to_token_stream()
        .into()
}
