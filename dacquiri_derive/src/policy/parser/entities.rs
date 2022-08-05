use syn::{
    Ident,
    Path,
    Token,
};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;


pub(crate) struct Entities {
    /*
        todo: allow marking entities as optional or mandatory

        Also, make sure to validate that all optional elements are not found in *all* branches
             ^-- maybe do this?

        proposed syntax:
        #[policy(
            entities = (
                user: Account?,         // <-- optional
                service: ServiceAuth?,  // <-- optional,
                bank: Bank              // <-- mandatory
            ),
            // ...
         )]

     */
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
    pub entity_type: Path,
}

impl Parse for EntityDeclaration {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let entity_name = input.parse()?;
        let _colon_token = input.parse()?;
        let entity_type = input.parse()?;

        let declaration = Self {
            entity_name,
            _colon_token,
            entity_type,
        };

        Ok(declaration)
    }
}