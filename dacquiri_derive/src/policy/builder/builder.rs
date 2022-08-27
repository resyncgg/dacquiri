use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::ops::Not;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{ConstParam, Generics, ItemTrait, TypeParamBound, LitStr};
use syn::punctuated::Punctuated;
use syn::{Token, parse_quote};
use crate::policy::builder::guard::GuardEntityPresence;
use crate::policy::parser::{EntityDeclaration, Policy};
use crate::policy::parser::clauses::Clause;
use thiserror::Error;


#[derive(Error, Debug)]
pub enum PolicyError {
    #[error("Auto traits are not supported.")]
    AutoTraitsNotSupported,
    #[error("Generic traits are not supported.")]
    GenericTraitsNotSupported,
    #[error("Policies that contain more than one guard may not use dependent policies. Offending dependent policy: `{0}`")]
    DependentPoliciesInUseOnMultiGuardPolicy(String)
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

        if let Err(err) = self.validate_guards_are_legal() {
            panic!("{err}");
        }

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

        /*
            The value of this implementation is that it allows policies with multiple guards
            to add implementations and keep the ability to call into their policy's code.

            The following is an example of what is not possible without this impl

            ```
            // assume this has two guards
            #[policy(..)]
            trait MyPolicy {
                fn do_thing(&self) { ... }
            }

            fn guarded(caller: impl MyPolicy) {
                let caller = caller.prove::<OtherAttribute, "entity">().unwrap();

                caller.do_thing(); // <-- this is an error since MyPolicy is no longer guaranteed
            }
            ```

            A fix for this is calling `MyPolicy::do_thing` before proving new attributes. Alternatively,
            figuring out what properties need to be re-proved and proving them will work too. Lastly,
            changing the `impl MyPolicy` to include the required HasConstraint will let Dacquiri implement the policy trait appropriately
         */
        // todo: Unstable! When this stops causing ICE, turn on by default
        #[cfg(feature = "unstable_policy_inheritance")]
        {
            // Prove ConstraintChain<_, _, _, MARKER> => MARKER
            tokens.extend(quote! {
                #[allow(non_upper_case_globals)]
                impl<
                    Next,
                    Attr,
                    #policy_const_generics_definition,
                    const STAG: EntityTag,
                    const RTAG: EntityTag
                > #policy_marker_ident #policy_const_generics_invocation for ConstraintChain<STAG, RTAG, Attr, Next>
                    where
                        Attr: BaseAttribute,
                        Next: #policy_marker_ident #policy_const_generics_invocation {}
            });
        }

        // implement 'policy marker' for 'guards'
        let all_entity_declarations = self.get_entities()
            .iter()
            .map(|declaration| {
                let name = declaration.entity_name.to_string();

                (name, declaration.clone())
            })
            .collect();

        for guard in &self.policy.guards {
            let entity_map = guard.generate_entity_requirement_map(self.get_entities());

            let guard_const_generics = guard.generate_const_generics(&entity_map);
            let policy_marker_const_generics = self.generate_policy_marker_const_generics_invoke(&entity_map);

            let guard_trait_bounds = guard.generate_guard_trait_bound(&all_entity_declarations, &entity_map);

            tokens.extend(quote! {
                #[allow(non_upper_case_globals)]
                impl<T, #guard_const_generics > #policy_marker_ident #policy_marker_const_generics for T
                    where
                        T: #guard_trait_bounds {}
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
        let common_clauses = self.calculate_policy_clauses_intersection();

        let mut entity_map = HashMap::new();

        for entity in self.get_entities() {
            let entity_name = entity.entity_name.to_string();

            if entity.is_optional {
                entity_map.insert(entity_name.clone(), GuardEntityPresence::Optional(entity_name.into()));
            } else {
                entity_map.insert(entity_name.clone(), GuardEntityPresence::Required(entity.clone()));
            }
        }

        for EntityDeclaration{ entity_name, entity_type, .. } in required_entities {
            trait_bound.push(parse_quote! {
                dacquiri::prelude::HasEntityWithType<#entity_name, #entity_type>
            });
        }

        let all_entity_declarations = self.get_entities()
            .iter()
            .map(|declaration| {
                let name = declaration.entity_name.to_string();

                (name, declaration.clone())
            })
            .collect();

        for clause in common_clauses {
            trait_bound.push(clause.generate_clause_trait_bound(&all_entity_declarations, &entity_map));
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
        trait_bound.extend(self.generate_required_entity_trait_bounds());

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

    fn generate_policy_marker_const_generics_invoke(&self, entity_map: &HashMap<String, GuardEntityPresence>) -> TokenStream {
        let const_generics_invoke = self.generate_const_generics(|entity_name| {
            match entity_map.get(&entity_name.to_string()) {
                Some(GuardEntityPresence::Required(EntityDeclaration { entity_name, .. })) => {

                    quote! { #entity_name }
                },
                Some(GuardEntityPresence::Optional(entity_ref)) => {
                    let entity_name_str = entity_ref.to_string();
                    let entity_name_lit_str = LitStr::new(&entity_name_str, Span::call_site());

                    quote! { #entity_name_lit_str }
                },
                None => unreachable!("Entity not found in entity_map")
            }
        });

        quote! { < #const_generics_invoke > }
    }

    fn validate_guards_are_legal(&self) -> Result<(), PolicyError> {
        // if the policy only has 1 guard, it's fine
        if self.policy.guards.len() == 1 {
            return Ok(());
        }

        // if this contains a policy, it's an illegal guard
        let dependent_policy = self.policy.guards.iter()
            .find_map(|guard| {
                guard.clauses().iter().find(|clause| matches!(clause, Clause::Policy(_)))
            });

        match dependent_policy {
            Some(Clause::Policy(policy)) => {
                Err(PolicyError::DependentPoliciesInUseOnMultiGuardPolicy(policy.to_string()))
            },
            _ => Ok(())
        }
    }

    /// checks for common constraints between all guards
    fn calculate_policy_clauses_intersection(&self) -> HashSet<Clause> {
        let common_constraints = self.policy.guards
            .iter()
            .map(|guard| {
                guard.clauses()
                    .iter()
                    .cloned()
                    .collect::<HashSet<_>>()
            })
            .reduce(|left, right| {
                left.intersection(&right)
                    .cloned()
                    .collect()
            });

        common_constraints.unwrap_or_default()
    }
}