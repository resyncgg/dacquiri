use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::Not;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{ConstParam, Generics, ItemTrait, TypeParamBound, LitStr};
use syn::punctuated::Punctuated;
use syn::{Token, parse_quote};
use crate::policy::builder::context::ContextEntityPresence;
use crate::policy::parser::{EntityDeclaration, Policy};


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
        let marker_trait_bounds = self.marker_trait_bounds();

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
            #[allow(non_upper_case_globals)]
            #[marker] pub trait #policy_marker_ident #policy_const_generics_with_defaults: #marker_trait_bounds {}
        });

        // implement 'policy' for all 'policy marker'
        tokens.extend(quote! {
            #[allow(non_upper_case_globals)]
            impl< T, #policy_const_generics_definition > #policy_ident #policy_const_generics_invocation for T
                where
                    T: #policy_marker_ident #policy_const_generics_invocation {}
        });

        // Prove EntityProof<_, _, MARKER> => MARKER
        tokens.extend(quote! {
            #[allow(non_upper_case_globals)]
            impl<
                Next,
                EntityType,
                #policy_const_generics_definition,
                const TAG: EntityTag
            > #policy_marker_ident #policy_const_generics_invocation for EntityProof<TAG, EntityType, Next>
                where
                    Next: #policy_marker_ident #policy_const_generics_invocation {}
        });

        // todo: Uncomment this once compiler bug is fixed - causes ICE
        // Prove ConstraintChain<_, _, _, MARKER> => MARKER
        // tokens.extend(quote! {
        //     #[allow(non_upper_case_globals)]
        //     impl<
        //         Next,
        //         Attr,
        //         #policy_const_generics_definition,
        //         const STAG: EntityTag,
        //         const RTAG: EntityTag
        //     > #policy_marker_ident #policy_const_generics_invocation for ConstraintChain<STAG, RTAG, Attr, Next>
        //         where
        //             Attr: BaseAttribute,
        //             Next: #policy_marker_ident #policy_const_generics_invocation {}
        // });

        // implement 'policy marker' for 'context's
        for context in &self.policy.contexts {
            let entity_map = context.generate_entity_requirement_map(self.get_entities());

            let context_const_generics = context.generate_const_generics(&entity_map);
            let policy_marker_const_generics = self.generate_policy_marker_const_generics_invoke(&entity_map);

            let context_trait_bounds = context.generate_context_trait_bound(&entity_map);

            tokens.extend(quote! {
                #[allow(non_upper_case_globals)]
                impl<T, #context_const_generics > #policy_marker_ident #policy_marker_const_generics for T
                    where
                        T: #context_trait_bounds {}
            });
        }
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

    /// Gets the defined entities of the policy
    fn get_entities(&self) -> &Vec<EntityDeclaration> {
        &self.policy
            .entities
            .declarations
    }

    fn get_required_entities(&self) -> Vec<&EntityDeclaration> {
        self.get_entities()
            .iter()
            .filter(|elem| elem.is_optional.not())
            .collect()
    }

    /// Generates the trait bounds required by all required entities
    fn generate_required_entity_trait_bounds(&self) -> Punctuated<TypeParamBound, Token![+]> {
        let mut trait_bound: Punctuated<TypeParamBound, Token![+]> = Punctuated::new();
        let required_entities: Vec<&EntityDeclaration> = self.get_required_entities();

        for EntityDeclaration{ entity_name, entity_type, .. } in required_entities {
            trait_bound.push(parse_quote! {
                dacquiri::prelude::HasEntityWithType<#entity_name, #entity_type>
            });
        }

        trait_bound
    }

    /// Generates the trait bounds found on a policy's definition
    fn policy_trait_bounds(&self) -> Punctuated<TypeParamBound, Token![+]> {
        let policy_marker_ident = self.policy_marker_ident();
        let policy_condition_const_generics = self.generate_const_generic_invocation();

        let mut trait_bound: Punctuated<TypeParamBound, Token![+]> = Punctuated::new();
        // Preserve any explicit trait bounds
        trait_bound.extend(self.item_trait.supertraits.clone());

        trait_bound.push(parse_quote! { #policy_marker_ident #policy_condition_const_generics });

        trait_bound
    }

    /// Generates the trait bounds found on the marker policy's definition
    fn marker_trait_bounds(&self) -> Punctuated<TypeParamBound, Token![+]> {
        let mut trait_bound: Punctuated<TypeParamBound, Token![+]> = Punctuated::new();

        trait_bound.push(parse_quote! { dacquiri::prelude::ConstraintT });
        trait_bound.push(parse_quote! { Sized });

        // Explicitly add HasConstraint bounds if only 1 context is specified to benefit from `impl <trait>` syntax
        match self.policy.contexts.first() {
            // todo: Update this to determined the min shared constraints across all contexts to share
            Some(context) if self.policy.contexts.len() == 1 => {
                let entity_map = context.generate_entity_requirement_map(self.get_entities());
                trait_bound.extend(context.generate_context_trait_bound(&entity_map));
            },
            _ => {
                trait_bound.extend(self.generate_required_entity_trait_bounds());
            }
        }

        trait_bound
    }

    /// Generates const generics based on all defined entities and a transform function
    fn generate_const_generics<F, O>(&self, transform: F) -> Punctuated<O, Token![,]>
        where
            F: Fn(&Ident) -> O
    {
        self.policy.entities.declarations
            .iter()
            .map(|EntityDeclaration { entity_name, .. }| transform(entity_name))
            .collect()
    }

    /// Generates const generics definition of the form `<const e1: &'static str, ...>`
    fn generate_const_generics_definition(&self) -> Punctuated<ConstParam, Token![,]> {
        self.generate_const_generics(|entity_name| {
            parse_quote! { const #entity_name: &'static str}
        })
    }

    /// Generates const generics definition with default token values of the form `<const e1: &'static str = "e1", ...>`
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

    fn generate_policy_marker_const_generics_invoke(&self, entity_map: &HashMap<String, ContextEntityPresence>) -> TokenStream {
        let const_generics_invoke = self.generate_const_generics(|entity_name| {
            match entity_map.get(&entity_name.to_string()) {
                Some(ContextEntityPresence::Required(EntityDeclaration { entity_name, .. })) => {

                    quote! { #entity_name }
                },
                Some(ContextEntityPresence::Optional(entity_ref)) => {
                    let entity_name_str = entity_ref.to_string();
                    let entity_name_lit_str = LitStr::new(&entity_name_str, Span::call_site());

                    quote! { #entity_name_lit_str }
                },
                None => unreachable!("Entity not found in entity_map")
            }
        });

        quote! { < #const_generics_invoke > }
    }
}