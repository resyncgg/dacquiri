use std::fmt::Debug;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{AttributeArgs, ItemTrait, NestedMeta, Meta, Path, TypeParamBound, Type, TypePath};
use syn::punctuated::Punctuated;
use syn::{Token, parse_quote};
use crate::requirement::{RequirementBound, RequirementBoundSet};

#[derive(Debug)]
pub enum RequirementError {
    AutoTraitsNotSupported,
    GenericTraitsNotSupported,
    UnsafeTraitNotSupported,
    BoundedTraitNotSupported,
    MissingRequirementBounds
}

pub struct RequirementBuilder {
    requirement_list: Vec<RequirementBound>,
    item_trait: ItemTrait
}

impl TryFrom<(RequirementBoundSet, ItemTrait)> for RequirementBuilder {
    type Error = RequirementError;

    fn try_from((requirement_args, item_trait): (RequirementBoundSet, ItemTrait)) -> Result<Self, Self::Error> {
        if item_trait.auto_token.is_some() {
            return Err(RequirementError::AutoTraitsNotSupported);
        }

        if !item_trait.generics.params.is_empty() {
            return Err(RequirementError::GenericTraitsNotSupported)
        }

        if item_trait.unsafety.is_some() {
            return Err(RequirementError::UnsafeTraitNotSupported);
        }

        if !item_trait.supertraits.is_empty() {
            return Err(RequirementError::BoundedTraitNotSupported);
        }

        // #[requirement(ChangeName, AccountEnabled)] => Some(vec![ChangeName, AccountEnabled])
        let meta_args: Option<Vec<RequirementBound>> = requirement_args.bounds.into_iter()
            .map(|meta| {
                match &meta {
                    RequirementBound { permission_ident, .. } => {
                        println!("Woah: {}", permission_ident);

                        Some(meta)
                    },
                    _ => None
                }
            })
            .collect();

        let requirement_list = meta_args.ok_or(RequirementError::MissingRequirementBounds)?;

        Ok(Self {
            requirement_list,
            item_trait
        })
    }
}

impl ToTokens for RequirementBuilder {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.item_trait.clone().into_token_stream());

        let trait_ident = &self.item_trait.ident;
        let trait_bound = self.generate_trait_bounds();

        tokens.extend(quote! {
            impl<T: #trait_bound> #trait_ident for T {}
        });
    }
}

impl RequirementBuilder {
    pub(crate) fn process(&mut self) {
        self.item_trait.supertraits = self.generate_trait_bounds();
    }

    fn generate_trait_bounds(&self) -> Punctuated<TypeParamBound, Token![+]> {
        let mut bound: Punctuated<TypeParamBound, Token![+]> = Punctuated::new();

        for requirement in &self.requirement_list {
            let req_name = &requirement.permission_ident;
            let id = match &requirement.specifier {
                Some(specifier) => {
                    let id = &specifier.id_lit;

                    quote! { #id }
                },
                None => {
                    quote!{ dacquiri::prelude::DEFAULT_GRANT_LABEL }
                }
            };

            let type_bound: TypeParamBound = parse_quote! {
                dacquiri::prelude::HasGrant<#req_name<{ #id }>, { #id }>
            };

            bound.push(type_bound);
        }

        bound
    }
}