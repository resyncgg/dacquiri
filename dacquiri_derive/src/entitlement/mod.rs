mod builder;
mod parser;

use syn::{ItemTrait, parse_macro_input, Token, Ident};
use proc_macro::TokenStream;
use proc_macro2::Literal;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use crate::entitlement::parser::EntitlementConstraints;

pub(crate) fn handle_entitlement(args: TokenStream, input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as ItemTrait);
    let entitlement_constraints = parse_macro_input!(args as EntitlementConstraints);

    for declaration in entitlement_constraints.element_declarations {
        println!(
            "Declaring {} as type {}",
            declaration.name.token().to_string(),
            declaration.element_type.to_string()
        );
    }

    for constraint in entitlement_constraints.constraints {
        match constraint.resource_constraint {
            Some(resource_constraint) => {
                println!(
                    "Constraining {} as having attribute {} on resource {}",
                    constraint.subject_id.token().to_string(),
                    constraint.attribute.to_string(),
                    resource_constraint.resource_id.token().to_string()
                );
            },
            None => {
                println!(
                    "Constraining {} as having attribute {}",
                    constraint.subject_id.token().to_string(),
                    constraint.attribute.to_string()
                );
            }
        }
    }

    //
    // let attribute_args = parse_macro_input!(args as RequirementBoundSet);
    //
    // let mut builder = RequirementBuilder::try_from((attribute_args, derive_input))
    //     .expect("Invalid use of the requirement macro.");
    //
    // builder.process();
    //
    // builder.to_token_stream().into()

    todo!()
}


//
// pub(crate) struct RequirementBoundSet {
//     bounds: Vec<RequirementBound>
// }
//
// impl Parse for RequirementBoundSet {
//     fn parse(input: ParseStream) -> syn::Result<Self> {
//         let mut bounds = Vec::new();
//
//         while let Ok(bound) = input.parse() {
//             bounds.push(bound);
//
//             // process next comma
//             let _ = input.parse::<Token![,]>();
//         }
//
//         Ok(RequirementBoundSet {
//             bounds
//         })
//     }
// }
//
// pub(crate) struct RequirementBound {
//     permission_ident: Ident,
//     specifier: Option<RequirementBoundSpecifier>,
// }
//
// impl Parse for RequirementBound {
//     fn parse(input: ParseStream) -> syn::Result<Self> {
//         let permission_ident: Ident = input.parse()?;
//         let specifier: Option<RequirementBoundSpecifier> = input.parse().ok();
//
//         Ok(RequirementBound {
//             permission_ident,
//             specifier
//         })
//     }
// }
//
// pub(crate) struct RequirementBoundSpecifier {
//     _as_token: Token![as],
//     id_lit: Literal
// }
//
// impl Parse for RequirementBoundSpecifier {
//     fn parse(input: ParseStream) -> syn::Result<Self> {
//         let _as_token: Token![as] = input.parse()?;
//         let id_lit: Literal = input.parse()?;
//
//         Ok(Self {
//             _as_token,
//             id_lit
//         })
//     }
// }