#![deny(warnings)]
#![allow(incomplete_features)]
#![feature(adt_const_params)]
#![feature(iter_intersperse)]

//! An authorization framework with compile-time enforcement.
//!
//! `Dacquiri-derive` makes using `Dacquiri` ergonomic.
//!
//! For more information on `Dacquiri`, check out its crate documentation!

extern crate core;

use proc_macro::TokenStream;

mod attribute;
mod policy;
mod utils;

#[proc_macro_attribute]
pub fn policy(args: TokenStream, input: TokenStream) -> TokenStream {
    policy::handle_policy(args, input)
}

#[proc_macro_attribute]
pub fn attribute(args: TokenStream, input: TokenStream) -> TokenStream {
    attribute::handle_attribute(args, input)
}
