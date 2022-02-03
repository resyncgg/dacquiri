#![feature(adt_const_params)]

//! An authorization framework with compile-time enforcement.
//!
//! `Dacquiri-derive` makes using `Dacquiri` ergonomic.
//!
//! For more information on `Dacquiri`, check out its crate documentation!

use proc_macro::TokenStream;

mod attribute;
mod entitlement;

#[proc_macro_attribute]
pub fn entitlement(args: TokenStream, input: TokenStream) -> TokenStream {
    entitlement::handle_entitlement(args, input)
}

#[proc_macro_attribute]
pub fn attribute(args: TokenStream, input: TokenStream) -> TokenStream {
    attribute::handle_attribute(args, input)
}
