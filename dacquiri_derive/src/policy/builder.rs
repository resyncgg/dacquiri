use std::fmt::Debug;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{ConstParam, Generics, ItemTrait, TypeParamBound, LitStr};
use syn::punctuated::Punctuated;
use syn::{Token, parse_quote};
use crate::policy::builder::PolicyError::AutoTraitsNotSupported;
use crate::policy::parser::{Constraint, DependentPolicy, EntityDeclaration, Policy};

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
            return Err(AutoTraitsNotSupported);
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
        let mut item_trait = self.item_trait.clone();

        let trait_ident = item_trait.ident.clone();
        let trait_bound = self.generate_trait_bounds();
        let const_bound_with_defaults = self.generate_const_generics(true);
        let const_bound_without_defaults = self.generate_const_generics(false);
        let const_generics_with_defaults: Generics = parse_quote! { < #const_bound_with_defaults > };
        let const_generics_invoke: Generics = self.generate_const_generics_invoke();

        item_trait.supertraits = trait_bound.clone();
        item_trait.generics = const_generics_with_defaults;

        tokens.extend(quote! {
            #[allow(non_upper_case_globals)]
            #item_trait
        });
        tokens.extend(quote! {
            #[allow(non_upper_case_globals)]
            impl<T, #const_bound_without_defaults > #trait_ident #const_generics_invoke for T
                where
                    T: #trait_bound {}
        });
    }
}

impl PolicyBuilder {
    fn generate_const_generics_invoke(&self) -> Generics {
        let const_generics_invoke: Punctuated<Ident, Token![,]> = self.policy
            .entities
            .declarations
            .iter()
            .map(|declaration| declaration.entity_name.clone())
            .collect();

        let generics_invoke = parse_quote! { < #const_generics_invoke > };

        generics_invoke
    }

    fn generate_const_generics(&self, with_defaults: bool) -> Punctuated<ConstParam, Token![,]> {
        let mut const_generics: Punctuated<ConstParam, Token![,]> = Punctuated::new();

        for entity in &self.policy.entities.declarations {
            let entity_name = &entity.entity_name;

            let const_bound = if with_defaults {
                let entity_name_str = entity_name.to_token_stream().to_string();
                let entity_name_lit_str = LitStr::new(&entity_name_str, Span::call_site());

                parse_quote! { const #entity_name: &'static str = #entity_name_lit_str }
            } else {
                parse_quote! { const #entity_name: &'static str}
            };

            const_generics.push(const_bound);
        }

        const_generics
    }

    fn generate_trait_bounds(&self) -> Punctuated<TypeParamBound, Token![+]> {
        let mut trait_bound: Punctuated<TypeParamBound, Token![+]> = Punctuated::new();
        // To allow for normal trait bounds for entitlements
        trait_bound.extend(self.item_trait.supertraits.clone());

        trait_bound.push(parse_quote! { Sized });
        trait_bound.push(parse_quote! { dacquiri::prelude::ConstraintT });

        if let Some(dependent_policies) = &self.policy.policies {
            for DependentPolicy { policy_name, entities } in &dependent_policies.policies {
                let punctuated_bounds: Punctuated<Ident, Token![,]> = entities
                    .clone()
                    .into_iter()
                    .collect();

                trait_bound.push(parse_quote! {
                    #policy_name<#punctuated_bounds>
                });
            }
        }

        if let Some(constraints) = &self.policy.constraints {
            for Constraint { subject_id, attribute, resource_constraint, .. } in &constraints.constraints {
                let resource = match resource_constraint {
                    Some(resource) => resource.resource_id.to_token_stream(),
                    None => parse_quote!{ { dacquiri::prelude::DEFAULT_ELEMENT_TAG } }
                };

                trait_bound.push(parse_quote! {
                    dacquiri::prelude::HasConstraint<#attribute, #subject_id, #resource>
                });
            }
        }

        for EntityDeclaration { entity_name, entity_type, .. } in &self.policy.entities.declarations {
            trait_bound.push(parse_quote! {
                dacquiri::prelude::HasEntityWithType<#entity_name, #entity_type>
            });
        }

        trait_bound
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