use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use syn::{
    Ident,
    Token,
};
use syn::parse::{Parse, ParseStream};
use crate::policy::entity_set::{EntityRef, EntitySet};
use crate::policy::parser::IsKeyword;


/// "User" is UserIsEnabled
/// "TeamA" is TeamIsEnabled
/// "User" is MemberOfTeam for "TeamA"
#[derive(Clone)]
pub(crate) struct Constraint {
    pub subject_id: Ident,
    _is_token: IsKeyword,
    pub attribute: Ident,
    pub resource_constraint: Option<ConstraintResource>
}

impl Eq for Constraint {}

impl PartialEq for Constraint {
    fn eq(&self, other: &Self) -> bool {
        let mut self_hasher = DefaultHasher::new();
        let mut other_hasher = DefaultHasher::new();

        self.hash(&mut self_hasher);
        other.hash(&mut other_hasher);

        self_hasher.finish() == other_hasher.finish()
    }
}

impl Hash for Constraint {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.subject_id.hash(state);
        self.attribute.hash(state);
        if let Some(resource) = &self.resource_constraint {
            resource.hash(state);
        }
    }
}

impl Parse for Constraint {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let subject_id = input.parse()?;
        let _is_token = input.parse()?;
        let attribute = input.parse()?;

        let resource_constraint = if input.peek(Token![,]) || input.is_empty() {
            None
        } else {
            Some(input.parse()?)
        };

        let constraint = Self {
            subject_id,
            _is_token,
            attribute,
            resource_constraint
        };

        Ok(constraint)
    }
}

/// for "TeamA"
#[derive(Clone)]
pub(crate) struct ConstraintResource {
    _for_token: Token![for],
    pub resource_id: Ident
}

impl Hash for ConstraintResource {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.resource_id.hash(state);
    }
}

impl Parse for ConstraintResource {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _for_token = input.parse()?;
        let resource_id = input.parse()?;

        Ok(Self {
            _for_token,
            resource_id
        })
    }
}

impl EntitySet for Constraint {
    fn entities(&self) -> HashSet<EntityRef> {
        let mut entities = HashSet::new();
        entities.insert(self.subject_id.to_string().into());

        if let Some(resource) = &self.resource_constraint {
            entities.insert(resource.resource_id.to_string().into());
        }

        entities
    }
}