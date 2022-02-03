use proc_macro2::{Group, TokenStream};
use quote::ToTokens;
use syn::{
    Ident,
    LitStr,
    Path,
    Token,
    parenthesized
};
use syn::token::Paren;
use syn::parse::{Parse, Parser, ParseStream};
use syn::punctuated::Punctuated;

const CONSTRAINTS_KEYWORD: &'static str = "constraints";
const IS_KEYWORD: &'static str = "is";

type ConstraintsKeyword = AtypicalKeyword<CONSTRAINTS_KEYWORD>;
type IsKeyword = AtypicalKeyword<IS_KEYWORD>;

pub(crate) struct EntitlementConstraints {
    pub(crate) element_declarations: Vec<ElementDeclaration>,
    pub(crate) constraints: Vec<Constraint>
}

impl Parse for EntitlementConstraints {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut element_declarations = Vec::new();

        while let Ok(declaration) = input.parse() {
            element_declarations.push(declaration);

            let _ = input.parse::<Token![,]>();
        }

        // constraints = ( ... )
        let _: ConstraintsKeyword = input.parse()?;
        let _: Token![=] = input.parse()?;

        let content;
        syn::parenthesized!(content in input);

        let punctuated_constraints: Punctuated<Constraint, Token![,]> = content.parse_terminated(Constraint::parse)?;
        let mut constraints = punctuated_constraints.into_iter().collect();

        let entitlement_constraints = Self {
            element_declarations,
            constraints
        };

        Ok(entitlement_constraints)
    }
}

struct AtypicalKeyword<const KEYWORD: &'static str> {
    _keyword: Path
}

impl<const KEYWORD: &'static str> Parse for AtypicalKeyword<KEYWORD> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _keyword = match input.parse::<Path>() {
            Ok(keyword) if keyword.to_token_stream().to_string() == KEYWORD => Ok(keyword),
            _ => {
                let error_msg = format!("Expected '{}' keyword.", KEYWORD);

                Err(syn::Error::new(input.span(), error_msg))
            }
        }?;

        Ok(Self {
            _keyword
        })
    }
}

/// "User" as User
/// "TeamA" as Team
/// "TeamB" as Team
pub(crate) struct ElementDeclaration {
    pub element_type: Ident,
    _as_token: Token![as],
    pub name: LitStr,
}

impl Parse for ElementDeclaration {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if !input.peek2(Token![as]) {
            return Err(syn::Error::new(input.span(), "Missing 'as' keyword in element declaration."));
        }

        let element_type = input.parse()?;
        let _as_token = input.parse()?;
        let name = input.parse()?;

        let declaration = Self {
            element_type,
            _as_token,
            name,
        };

        Ok(declaration)
    }
}

/// "User" as UserIsEnabled
/// "TeamA" as TeamIsEnabled
/// "User" as MemberOfTeam for "TeamA"
pub(crate) struct Constraint {
    pub subject_id: LitStr,
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
    pub resource_id: LitStr
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