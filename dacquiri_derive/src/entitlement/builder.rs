use std::fmt::Debug;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{ItemTrait, TypeParamBound};
use syn::punctuated::Punctuated;
use syn::{Token, parse_quote};
use crate::entitlement::{RequirementBound, RequirementBoundSet};

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

        if requirement_args.bounds.is_empty() {
            return Err(RequirementError::MissingRequirementBounds);
        }

        Ok(Self {
            requirement_list: requirement_args.bounds,
            item_trait
        })
    }
}

impl ToTokens for RequirementBuilder {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let trait_ident = &self.item_trait.ident;
        let trait_bound = self.generate_trait_bounds();

        tokens.extend(self.item_trait.clone().into_token_stream());
        tokens.extend(quote! {
            impl<T> #trait_ident for T
                where
                    T: #trait_bound {}
        });
    }
}

impl RequirementBuilder {
    pub(crate) fn process(&mut self) {
        self.item_trait.supertraits = self.generate_trait_bounds();
    }

    fn generate_trait_bounds(&self) -> Punctuated<TypeParamBound, Token![+]> {
        let mut bound: Punctuated<TypeParamBound, Token![+]> = Punctuated::new();
        bound.push(parse_quote! { Sized });
        bound.push(parse_quote! { dacquiri::prelude::AttributeChainT });

        for requirement in &self.requirement_list {
            let req_name = &requirement.permission_ident;
            let id = match &requirement.specifier {
                Some(specifier) => {
                    let id = &specifier.id_lit;

                    quote! { #id }
                },
                None => {
                    quote!{ dacquiri::prelude::DEFAULT_ATTRIBUTE_TAG }
                }
            };

            let type_bound: TypeParamBound = parse_quote! {
                dacquiri::prelude::HasAttribute<#req_name<{ #id }>, { #id }>
            };

            bound.push(type_bound);
        }

        bound
    }
}