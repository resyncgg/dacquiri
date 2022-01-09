use std::fmt::Debug;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{AttributeArgs, ItemTrait, NestedMeta, Meta, Path, TypeParamBound};
use syn::punctuated::Punctuated;
use syn::{Token, parse_quote};

#[derive(Debug)]
pub enum RequirementError {
    AutoTraitsNotSupported,
    GenericTraitsNotSupported,
    UnsafeTraitNotSupported,
    BoundedTraitNotSupported,
    MissingRequirementBounds
}

pub struct RequirementBuilder {
    requirement_list: Vec<Path>,
    item_trait: ItemTrait
}

impl TryFrom<(AttributeArgs, ItemTrait)> for RequirementBuilder {
    type Error = RequirementError;

    fn try_from((requirement_args, item_trait): (AttributeArgs, ItemTrait)) -> Result<Self, Self::Error> {
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
        let meta_args: Option<Vec<Path>> = requirement_args.into_iter()
            .map(|meta| match meta {
                NestedMeta::Meta(Meta::Path(bound)) => Some(bound),
                _ => None
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
            let type_bound: TypeParamBound = parse_quote! {
                dacquiri::prelude::HasGrant<#requirement>
            };

            bound.push(type_bound);
        }

        bound
    }
}