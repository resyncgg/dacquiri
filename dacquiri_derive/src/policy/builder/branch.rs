use std::collections::HashMap;
use syn::TypeParamBound;
use syn::ConstParam;
use syn::punctuated::Punctuated;
use syn::{Token, parse_quote};
use crate::policy::entity_set::{EntityRef, EntitySet};
use crate::policy::parser::branch::Branch;
use crate::policy::parser::EntityDeclaration;

pub(crate) enum BranchEntityPresence {
    Required(EntityDeclaration),
    Optional(EntityRef)
}

impl Branch {
    pub(crate) fn generate_entity_requirement_map(
        &self,
        policy_entities: &Vec<EntityDeclaration>
    ) -> HashMap<String, BranchEntityPresence> {
        let mut branch_entity_map = HashMap::new();
        // provides a map for all entities
        let policy_entity_map: HashMap<String, EntityDeclaration> = policy_entities.iter()
            .map(|entity| {
                let entity_name = entity.entity_name.to_string();

                (entity_name, entity.clone())
            })
            .collect();

        for (name, entity) in &policy_entity_map {
            if entity.is_optional {
                let entity_name = entity.entity_name.to_string();

                branch_entity_map.insert(name.clone(), BranchEntityPresence::Optional(entity_name.into()));
            } else {
                branch_entity_map.insert(name.clone(), BranchEntityPresence::Required(entity.clone()));
            }
        }

        for branch_entity in self.entities() {
            let entity_name = branch_entity.to_string();
            let entity_definition = policy_entity_map
                .get(&entity_name)
                .expect("Missing entity definition on policy");

            branch_entity_map.insert(entity_name, BranchEntityPresence::Required(entity_definition.clone()));
        }

        branch_entity_map
    }

    pub(crate) fn generate_const_generics(&self, entity_map: &HashMap<String, BranchEntityPresence>) -> Punctuated<ConstParam, Token![,]> {
        let mut const_generics = Punctuated::new();

        for (_, entity_presence) in entity_map {
            if let BranchEntityPresence::Required(EntityDeclaration { entity_name, .. }) = entity_presence {
                const_generics.push(parse_quote! {
                    const #entity_name: &'static str
                });
            }
        }

        const_generics
    }

    pub(crate) fn generate_branch_trait_bound(&self, entity_map: &HashMap<String, BranchEntityPresence>) -> Punctuated<TypeParamBound, Token![+]> {
        let mut trait_bound: Punctuated<TypeParamBound, Token![+]> = Punctuated::new();
        trait_bound.push(parse_quote! { dacquiri::prelude::ConstraintT });

        for clause in self.clauses() {
            trait_bound.push(clause.generate_clause_trait_bound(&entity_map));
        }

        for (_, entity_presence) in entity_map {
            if let BranchEntityPresence::Required(EntityDeclaration { entity_name, entity_type, .. }) = entity_presence {
                trait_bound.push(parse_quote! {
                    dacquiri::prelude::HasEntityWithType<#entity_name, #entity_type>
                });

                // todo: is this needed?
                trait_bound.push(parse_quote! {
                    dacquiri::prelude::HasEntity<#entity_name>
                });
            }
        }

        trait_bound
    }
}