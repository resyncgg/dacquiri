use proc_macro2::{TokenStream, Ident, Span};
use quote::{ToTokens, quote};
use syn::{AttributeArgs, ItemFn, NestedMeta, Meta, Path, Block, ReturnType, Type, TypePath, PathArguments, parse_quote, FnArg, PatType, GenericArgument, Pat};

#[derive(Debug)]
pub enum GrantError {
    MissingGrantName,
    GenericFunctionsNotSupported,
    VariadicFunctionsNotSupported,
    UnsafeFunctionsNotSupported,
    ExternFunctionsNotSupported,
    AsyncFunctionsNotSupported,
    ConstFunctionsNotSupported,
    FunctionReturnTypeRequired,
    IncorrectFunctionReturnType,
    ReturnTypeMustBeGrantResult,
    GrantResultRequiresOneGeneric,
    IncorrectNumberOfInputArguments,
    IncorrectErrorType,
    IncorrectPrincipalType,
    IncorrectResourceType,
}

pub struct GrantBuilder {
    permission_name: Path,
    principal_type: Type,
    resource_type: Type,
    error_type: Type,
    grant_check_block: Block,
    principal_var: Ident,
    resource_var: Ident
}

impl TryFrom<(AttributeArgs, ItemFn)> for GrantBuilder {
    type Error = GrantError;

    fn try_from((mut args, grant_check_fn): (AttributeArgs, ItemFn)) -> Result<Self, Self::Error> {
        // #[grant(AccountEnabled)] => AccountEnabled
        let permission_name = match args.pop() {
            Some(NestedMeta::Meta(Meta::Path(bound))) => bound,
            _ => return Err(GrantError::MissingGrantName)
        };

        let grant_check_fn = match grant_check_fn {
            _ if !grant_check_fn.sig.generics.params.is_empty() => Err(GrantError::GenericFunctionsNotSupported),
            _ if grant_check_fn.sig.variadic.is_some() => Err(GrantError::VariadicFunctionsNotSupported),
            _ if grant_check_fn.sig.unsafety.is_some() => Err(GrantError::UnsafeFunctionsNotSupported),
            _ if grant_check_fn.sig.abi.is_some() => Err(GrantError::ExternFunctionsNotSupported),
            _ if grant_check_fn.sig.asyncness.is_some() => Err(GrantError::AsyncFunctionsNotSupported),
            _ if grant_check_fn.sig.constness.is_some() => Err(GrantError::ConstFunctionsNotSupported),
            _ => Ok(grant_check_fn),
        }?;

        let inner = match grant_check_fn.clone().sig.output {
            ReturnType::Type(_, inner) => Ok(inner),
            _ => Err(GrantError::FunctionReturnTypeRequired)
        }?;

        let mut segments = match *inner {
            Type::Path(TypePath { path, .. }) => { Ok(path.segments) }
            _ => Err(GrantError::IncorrectFunctionReturnType)
        }?;

        let first_segment = segments.pop().ok_or(GrantError::IncorrectFunctionReturnType)?;

        let grant_result_ident: Ident = parse_quote!(GrantResult);

        let error_type = match first_segment.value() {
            path_segment if path_segment.ident != grant_result_ident => Err(GrantError::ReturnTypeMustBeGrantResult),
            path_segment => match path_segment.arguments.clone() {
                PathArguments::AngleBracketed(mut arguments) if arguments.args.len() == 1 => {
                    match arguments.args.pop().map(|pair| pair.into_value()) {
                        Some(GenericArgument::Type(error_type)) => Ok(error_type),
                        _ => Err(GrantError::IncorrectErrorType)
                    }
                },
                _ => Err(GrantError::GrantResultRequiresOneGeneric)
            }
        }?;

        let mut inputs = grant_check_fn.sig.inputs.clone().into_iter();

        let (principal_var, principal_type) = match inputs.next() {
            Some(FnArg::Typed(PatType { pat, ty, .. })) => {
                match (*pat, *ty) {
                    (Pat::Wild(_), path) => {
                        let ident: Ident = Ident::new("_", Span::call_site());
                        Ok((ident, path))
                    },
                    (Pat::Ident(var_name), path) => Ok((var_name.ident, path)),
                    _ => Err(GrantError::IncorrectPrincipalType)
                }
            },
            _ => Err(GrantError::IncorrectNumberOfInputArguments)
        }?;

        let (resource_var, resource_type) = match inputs.next() {
            Some(FnArg::Typed(PatType { pat, ty, .. })) => {
                match (*pat, *ty) {
                    (Pat::Ident(var_name), path) => Ok((var_name.ident, path)),
                    _ => Err(GrantError::IncorrectResourceType)
                }
            },
            None => {
                let ty: Type = parse_quote! { () };
                let default_resource_var = Ident::new("resource", Span::call_site());
                Ok((default_resource_var, ty))
            },
            _ => Err(GrantError::IncorrectResourceType)
         }?;

        let grant_check_block = *grant_check_fn.block;

        Ok(GrantBuilder {
            permission_name,
            principal_type,
            resource_type,
            error_type,
            grant_check_block,
            principal_var,
            resource_var
        })
    }
}

impl ToTokens for GrantBuilder {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let permission_identity = &self.permission_name;
        let resource_type = &self.resource_type;
        let principal_type = &self.principal_type;
        let error_type = &self.error_type;
        let grant_check_impl = &self.grant_check_block;

        let principal_var_name = &self.principal_var;
        let resource_var_name = &self.resource_var;

        tokens.extend(quote!{
            pub struct #permission_identity<const ID: &'static str = { dacquiri::prelude::DEFAULT_GRANT_LABEL }>(#resource_type);
        });

        tokens.extend(quote!{
           impl<const ID: &'static str> dacquiri::prelude::Grant<ID> for #permission_identity<ID> {
                type Principal = #principal_type;
                type Resource = #resource_type;
                type Error = #error_type;

                fn new_with_resource(resource: Self::Resource) -> Self { Self(resource) }
                fn get_resource(&self) -> &Self::Resource { &self.0 }

                // all users can change their name
                fn check_grant(#principal_var_name: &Self::Principal, #resource_var_name: &Self::Resource) -> dacquiri::prelude::GrantResult<Self::Error> #grant_check_impl
            }
        });
    }
}