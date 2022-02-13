#![deny(warnings)]
#![allow(incomplete_features)]
#![feature(adt_const_params)]
#![feature(associated_type_defaults)]
#![feature(generic_arg_infer)]
#![feature(generic_associated_types)]
#![feature(trait_alias)]
#![feature(marker_trait_attr)]
#![feature(explicit_generic_args_with_impl_trait)]
#![feature(negative_impls)]
#![feature(auto_traits)]

#![doc = include_str!("../README.md")]

mod error;
mod attribute;
mod chain;
mod acquire;
mod has;
pub mod prelude;
mod constraint;
mod store;
mod private;

#[doc(hidden)]
pub const DEFAULT_ELEMENT_TAG: &str = "___";


