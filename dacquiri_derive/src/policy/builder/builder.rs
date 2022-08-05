use std::fmt::Debug;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{ConstParam, Generics, ItemTrait, TypeParamBound, LitStr};
use syn::punctuated::Punctuated;
use syn::{Token, parse_quote};
use crate::policy::entity_set::EntitySet;
use crate::policy::parser::{EntityDeclaration, Policy};
use crate::policy::parser::context::Context;


#[derive(Debug)]
pub enum PolicyError {
    AutoTraitsNotSupported,
    GenericTraitsNotSupported
}

pub struct PolicyBuilder {
    policy: Policy,
    item_trait: ItemTrait
}

impl TryFrom<(Policy, ItemTrait)> for PolicyBuilder {
    type Error = PolicyError;

    fn try_from((policy, item_trait): (Policy, ItemTrait)) -> Result<Self, Self::Error> {
        if item_trait.auto_token.is_some() {
            return Err(PolicyError::AutoTraitsNotSupported);
        }

        if !item_trait.generics.params.is_empty() {
            return Err(PolicyError::GenericTraitsNotSupported);
        }

        let builder = PolicyBuilder {
            policy,
            item_trait
        };

        Ok(builder)
    }
}

impl ToTokens for PolicyBuilder {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut policy_trait = self.item_trait.clone();
        let policy_ident = self.policy_ident();
        let policy_marker_ident = self.policy_marker_ident();
        let policy_trait_bounds = self.policy_trait_bounds();
        let policy_common_entity_trait_bounds = self.policy_common_entity_trait_bounds();
        let policy_const_generics_invocation = self.generate_const_generic_invocation();
        let policy_const_generics_definition = self.generate_const_generics_definition();
        let policy_const_generics_with_defaults: Generics = {
            let generics = self.generate_const_generics_definition_with_defaults();

            parse_quote! { < #generics > }
        };

        policy_trait.supertraits = policy_trait_bounds.clone();
        policy_trait.generics = policy_const_generics_with_defaults.clone();

        // write the policy definition
        tokens.extend(quote! {
            #[allow(non_upper_case_globals)]
            #policy_trait
        });

        // write the marker trait
        tokens.extend(quote! {
            #[marker] pub trait #policy_marker_ident #policy_const_generics_with_defaults: #policy_common_entity_trait_bounds {}
        });

        // implement 'policy' for all 'policy marker'
        tokens.extend(quote!{
            impl<T, #policy_const_generics_definition > #policy_ident #policy_const_generics_invocation for T
                where
                    T: #policy_marker_ident #policy_const_generics_invocation {}
        });

        // implement 'policy marker' for 'context's
        for context in &self.policy.contexts {
            let mut context_trait_bounds = context.generate_context_trait_bound();
            context_trait_bounds.extend(policy_common_entity_trait_bounds.clone());

            tokens.extend(quote! {
                impl<T, #policy_const_generics_definition > #policy_marker_ident #policy_const_generics_invocation for T
                    where
                        T: #context_trait_bounds {}
            });
        }
        //
        // tokens.extend(quote! {
        //     #[allow(non_upper_case_globals)]
        //     impl<T, #const_bound_without_defaults > #policy_ident #const_generics_invoke for T
        //         where
        //             T: #policy_trait_bounds {}
        // });
    }
}

impl PolicyBuilder {
    /// The ident of the policy trait
    fn policy_ident(&self) -> Ident {
        self.item_trait.ident.clone()
    }

    /// The ident of the policy's condition trait
    fn policy_marker_ident(&self) -> Ident {
        let condition_name = format!("{}Marker", self.policy_ident().to_string());

        Ident::new(&condition_name, self.policy_ident().span())
    }

    /// Filters a policy's entity declarations to only include the common entities. Useful for trait bounds
    fn common_entity_declarations(&self) -> Vec<&EntityDeclaration> {
        let common_entities = self.policy.common_entities();

        self.policy.entities.declarations
            .iter()
            .filter(|EntityDeclaration { entity_name, .. }| {
                let entity_ref = entity_name.to_string().into();

                common_entities.contains(&entity_ref)
            })
            .collect()
    }

    fn policy_common_entity_trait_bounds(&self) -> Punctuated<TypeParamBound, Token![+]> {
        let mut trait_bound: Punctuated<TypeParamBound, Token![+]> = Punctuated::new();

        for EntityDeclaration { entity_name, entity_type, .. } in &self.common_entity_declarations() {
            trait_bound.push(parse_quote! {
                dacquiri::prelude::HasEntityWithType<#entity_name, #entity_type>
            });
        }

        trait_bound.push(parse_quote! { dacquiri::prelude::ConstraintT });

        trait_bound
    }

    /// Generates the trait bounds found on a policy's definition
    fn policy_trait_bounds(&self) -> Punctuated<TypeParamBound, Token![+]> {
        let policy_marker_ident = self.policy_marker_ident();
        let policy_condition_const_generics = self.generate_const_generic_invocation();

        let mut trait_bound: Punctuated<TypeParamBound, Token![+]> = Punctuated::new();
        // Preserve any explicit trait bounds
        trait_bound.extend(self.item_trait.supertraits.clone());

        trait_bound.push(parse_quote! { Sized });
        trait_bound.push(parse_quote! { dacquiri::prelude::ConstraintT });
        trait_bound.push(parse_quote! { #policy_marker_ident #policy_condition_const_generics });

        for EntityDeclaration { entity_name, entity_type, .. } in &self.common_entity_declarations() {
            trait_bound.push(parse_quote! {
                dacquiri::prelude::HasEntityWithType<#entity_name, #entity_type>
            });
        }

        trait_bound
    }

    fn generate_const_generics<F, O>(&self, transform: F) -> Punctuated<O, Token![,]>
        where
            F: Fn(&Ident) -> O
    {
        self.policy.entities.declarations
            .iter()
            .map(|EntityDeclaration { entity_name, .. }| transform(entity_name))
            .collect()
    }

    fn generate_const_generics_definition(&self) -> Punctuated<ConstParam, Token![,]> {
        self.generate_const_generics(|entity_name| {
            parse_quote! { const #entity_name: &'static str}
        })
    }

    fn generate_const_generics_definition_with_defaults(&self) -> Punctuated<ConstParam, Token![,]> {
        self.generate_const_generics(|entity_name| {
            let entity_name_str = entity_name.to_token_stream().to_string();
            let entity_name_lit_str = LitStr::new(&entity_name_str, Span::call_site());

            parse_quote! { const #entity_name: &'static str = #entity_name_lit_str }
        })
    }

    fn generate_const_generic_invocation(&self) -> Generics {
        let const_generics_invoke = self.generate_const_generics(|entity_name| entity_name.clone());

        parse_quote! { < #const_generics_invoke > }
    }
}

//
// impl ToTokens for RequirementBuilder {
//     fn to_tokens(&self, tokens: &mut TokenStream) {
//         let trait_ident = &self.item_trait.ident;
//         let trait_bound = self.generate_trait_bounds();
//
//         tokens.extend(self.item_trait.clone().into_token_stream());
//         tokens.extend(quote! {
//             impl<T> #trait_ident for T
//                 where
//                     T: #trait_bound {}
//         });
//     }
// }
//
// impl RequirementBuilder {
//     pub(crate) fn process(&mut self) {
//         self.item_trait.supertraits = self.generate_trait_bounds();
//     }
//
//     fn generate_trait_bounds(&self) -> Punctuated<TypeParamBound, Token![+]> {
//         let mut bound: Punctuated<TypeParamBound, Token![+]> = Punctuated::new();
//         // To allow for normal trait bounds for entitlements
//         bound.extend(self.item_trait.supertraits.clone());
//
//         bound.push(parse_quote! { Sized });
//         bound.push(parse_quote! { dacquiri::prelude::AttributeChainT });
//
//         for requirement in &self.requirement_list {
//             let req_name = &requirement.permission_ident;
//             let id = match &requirement.specifier {
//                 Some(specifier) => {
//                     let id = &specifier.id_lit;
//
//                     quote! { #id }
//                 },
//                 None => {
//                     quote!{ dacquiri::prelude::DEFAULT_ATTRIBUTE_TAG }
//                 }
//             };
//
//             let type_bound: TypeParamBound = parse_quote! {
//                 dacquiri::prelude::HasAttribute<#req_name<{ #id }>, { #id }>
//             };
//
//             bound.push(type_bound);
//         }
//
//         bound
//     }
// }
