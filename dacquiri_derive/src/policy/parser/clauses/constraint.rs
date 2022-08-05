use syn::{
    Ident,
    Token,
};
use syn::parse::{Parse, ParseStream};
use crate::policy::parser::IsKeyword;


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