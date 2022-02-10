use proc_macro2::Span;
use quote::ToTokens;
use syn::{
    Ident,
    Path,
    Token,
};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use crate::utils::NonstandardKeyword;

const ENTITIES_KEYWORD: &str = "entities";
const POLICIES_KEYWORD: &str = "policies";
const CONSTRAINTS_KEYWORD: &str = "constraints";
const IS_KEYWORD: &str = "is";

type IsKeyword = NonstandardKeyword<IS_KEYWORD>;

/**
Example

#[policy(
    entities = (
        user: User,
        team_one: Team,
        team_two: Team,
    ),
    policies = (
        ActiveTeamMember(user, team_one),
        ActiveTeamMember(user, team_two)
    ),
)]
pub trait MultiTeamMember {
    fn transfer_data(&self) {
        let team_ones_files = self.as_policy::<ActiveTeamMember<user, team_one>>().get_files(); // team one

        self.as_policy::<ActiveTeamMember<user, team_two>>().add_files(team_ones_files); // team two
    }
}

#[policy(
    entities = (
        user: User,
        team: Team
    ),
    constraints = (
        user is EnabledUser,
        team is EnabledTeam,
        user is TeamMember for team,
    )
)]
pub trait ActiveTeamMember {
    fn get_files(&self) -> Vec<File> { /* .. */ }
    fn add_files(&self, files: Vec<File>) { /* .. */ }
}
*/

enum PolicyKey {
    Entities,
    Policies,
    Constraints
}

impl TryFrom<Ident> for PolicyKey {
    type Error = ();

    fn try_from(value: Ident) -> Result<Self, Self::Error> {
        match value.to_token_stream().to_string() {
            token if token == ENTITIES_KEYWORD => Ok(PolicyKey::Entities),
            token if token == POLICIES_KEYWORD => Ok(PolicyKey::Policies),
            token if token == CONSTRAINTS_KEYWORD => Ok(PolicyKey::Constraints),
            _ => Err(())
        }
    }
}

impl Parse for PolicyKey {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let value: Ident = input.parse()?;

        value
            .try_into()
            .map_err(|_| {
                syn::Error::new(Span::call_site(), "Keyword was invalid.")
            })
    }
}

pub(crate) struct Policy {
    pub(crate) entities: Entities,
    pub(crate) policies: Option<DependentPolicies>,
    pub(crate) constraints: Option<Constraints>
}

impl Parse for Policy {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut maybe_entities: Option<Entities> = None;
        let mut policies: Option<DependentPolicies> = None;
        let mut constraints: Option<Constraints> = None;

        // only up-to three fields to parse
        while let Ok(policy_key) = input.parse() {
            let _ = input.parse::<Token![=]>()?;

            match policy_key {
                PolicyKey::Entities => maybe_entities = Some(input.parse()?),
                PolicyKey::Policies => policies = Some(input.parse()?),
                PolicyKey::Constraints => constraints = Some(input.parse()?)
            }

            if input.peek(Token![,]) {
                let _ = input.parse::<Token![,]>()?;
            }
        }

        let entities = maybe_entities
            .ok_or_else(|| syn::Error::new(
                Span::call_site(),
                "Entities are required to be defined for a policy."
            ))?;

        let policy = Policy {
            entities,
            policies,
            constraints
        };

        Ok(policy)
    }
}

pub(crate) struct Entities {
    pub(crate) declarations: Vec<EntityDeclaration>
}

impl Parse for Entities {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        syn::parenthesized!(content in input);

        let punctuated_entities: Punctuated<EntityDeclaration, Token![,]> = content.parse_terminated(EntityDeclaration::parse)?;
        let declarations = punctuated_entities.into_iter().collect();

        Ok(Self {
            declarations
        })
    }
}

/// user: User
/// team_a: Team
/// team_b: Team
pub(crate) struct EntityDeclaration {
    pub entity_name: Ident,
    _colon_token: Token![:],
    _entity_type: Path,
}

impl Parse for EntityDeclaration {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let entity_name = input.parse()?;
        let _colon_token = input.parse()?;
        let _entity_type = input.parse()?;

        let declaration = Self {
            entity_name,
            _colon_token,
            _entity_type,
        };

        Ok(declaration)
    }
}

pub(crate) struct DependentPolicies {
    pub(crate) policies: Vec<DependentPolicy>
}

impl Parse for DependentPolicies {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        syn::parenthesized!(content in input);

        let punctuated_policies: Punctuated<DependentPolicy, Token![,]> = content.parse_terminated(DependentPolicy::parse)?;
        let policies = punctuated_policies.into_iter().collect();

        Ok(Self {
            policies
        })
    }
}

pub(crate) struct DependentPolicy {
    pub policy_name: Ident,
    pub entities: Vec<Ident>
}

impl Parse for DependentPolicy {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let policy_name = input.parse()?;

        let content;
        syn::parenthesized!(content in input);

        let punctuated_entities: Punctuated<Ident, Token![,]> = content.parse_terminated(Ident::parse)?;
        let entities = punctuated_entities.into_iter().collect();

        Ok(Self {
            policy_name,
            entities
        })
    }
}

pub(crate) struct Constraints {
    pub(crate) constraints: Vec<Constraint>
}

impl Parse for Constraints {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        syn::parenthesized!(content in input);

        let punctuated_constraints: Punctuated<Constraint, Token![,]> = content.parse_terminated(Constraint::parse)?;
        let constraints = punctuated_constraints.into_iter().collect();

        Ok(Self {
            constraints
        })
    }
}

/// "User" is UserIsEnabled
/// "TeamA" is TeamIsEnabled
/// "User" is MemberOfTeam for "TeamA"
pub(crate) struct Constraint {
    pub subject_id: Ident,
    _is_token: IsKeyword,
    pub attribute: Ident,
    pub resource_constraint: Option<ConstraintResource>
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
pub(crate) struct ConstraintResource {
    _for_token: Token![for],
    pub resource_id: Ident
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