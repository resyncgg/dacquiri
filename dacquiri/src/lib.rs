#![allow(incomplete_features)]
#![feature(adt_const_params)]
#![feature(negative_impls)]
#![feature(auto_traits)]
#![feature(trait_alias)]
#![feature(associated_type_defaults)]
#![feature(explicit_generic_args_with_impl_trait)]
#![feature(generic_arg_infer)]
#![feature(generic_associated_types)]
#![feature(specialization)]

pub mod prelude;
mod grant;
mod principal;
mod grant_chain;

/// Is the label applied to grant checks that are not resource dependant.
pub const DEFAULT_GRANT_TAG: &'static str = "GrantCheck::__";