use proc_macro2::Span;
use quote::ToTokens;
use syn::{
    Ident,
    Token,
};
use syn::parse::{Parse, ParseStream};

use super::{
    ENTITIES_KEYWORD,
    BRANCH_KEYWORD,
    entities::Entities,
    branch::Branch,
};


/**
Example

#[policy(
    entities = (
        user: User,
        team_one: Team,
        team_two: Team,
    ),
    branch = (
        ActiveTeamMember(user, team_one),
        ActiveTeamMember(user, team_two)
    ),
    branch = (
        user is PlatformAdmin
    )
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
    branch = (
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
    Branch
}

impl TryFrom<Ident> for PolicyKey {
    type Error = ();

    fn try_from(value: Ident) -> Result<Self, Self::Error> {
        match value.to_token_stream().to_string() {
            token if token == ENTITIES_KEYWORD => Ok(PolicyKey::Entities),
            token if token == BRANCH_KEYWORD => Ok(PolicyKey::Branch),
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
    pub(crate) branches: Vec<Branch>
}

impl Parse for Policy {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut maybe_entities: Option<Entities> = None;
        let mut branches: Vec<Branch> = Vec::new();

        while let Ok(policy_key) = input.parse() {
            let _ = input.parse::<Token![=]>()?;

            match policy_key {
                PolicyKey::Entities => maybe_entities = Some(input.parse()?),
                PolicyKey::Branch => branches.push(input.parse()?),
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
            branches
        };

        Ok(policy)
    }
}