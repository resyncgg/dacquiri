use std::collections::HashMap;
use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::TypeParamBound;
use syn::punctuated::Punctuated;
use syn::{Token, parse_quote, LitStr};
use crate::policy::builder::branch::BranchEntityPresence;
use crate::policy::parser::clauses::{Clause, Constraint, DependentPolicy};
use crate::policy::parser::EntityDeclaration;

impl Clause {
    pub(crate) fn generate_clause_trait_bound(&self, entity_map: &HashMap<String, BranchEntityPresence>) -> TypeParamBound {
        match self {
            Clause::Constraint(Constraint { subject_id, attribute, resource_constraint, .. }) => {
                let resource = match resource_constraint {
                    Some(resource) => resource.resource_id.to_token_stream(),
                    None => parse_quote!{ { dacquiri::prelude::DEFAULT_ELEMENT_TAG } }
                };

                let output = parse_quote! {
                    dacquiri::prelude::HasConstraint<#attribute, #subject_id, #resource>
                };

                output
            },
            Clause::Policy(DependentPolicy { policy_name, entities }) => {
                let mut punctuated_bounds: Punctuated<TokenStream, Token![,]> = Punctuated::new();

                for entity in entities {
                    let entity_name = entity.to_string();
                    let component = match entity_map.get(&entity_name) {
                        Some(BranchEntityPresence::Required(EntityDeclaration { entity_name, .. })) => {
                            quote! { #entity_name }
                        },
                        Some(BranchEntityPresence::Optional(entity_ref)) => {
                            let entity_name_str = entity_ref.to_string();
                            let entity_name_lit_str = LitStr::new(&entity_name_str, Span::call_site());

                            quote! { #entity_name_lit_str }
                        },
                        None => unreachable!("Entity not found in entity_map")
                    };

                    punctuated_bounds.push(component);
                }

                let output = parse_quote! {
                    #policy_name<#punctuated_bounds>
                };

                output
            }
        }
    }
}
