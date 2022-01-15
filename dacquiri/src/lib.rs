#![allow(incomplete_features)]
#![feature(adt_const_params)]
#![feature(negative_impls)]
#![feature(auto_traits)]
#![feature(trait_alias)]
#![feature(associated_type_defaults)]
#![feature(generic_arg_infer)]
#![feature(generic_associated_types)]

#![doc = include_str!("../README.md")]

pub mod prelude;
mod subject;
mod attributes;
mod attribute_chain;
mod resource;

#[doc(hidden)]
pub const DEFAULT_ATTRIBUTE_TAG: &'static str = "AttributeTag::DEFAULT";
