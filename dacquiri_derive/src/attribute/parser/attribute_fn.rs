use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{Block, FnArg, GenericArgument, ItemFn, Lifetime, parse_quote, Pat, Path, PathArguments, PatType, ReturnType, Type, TypePath};
use crate::attribute::builder::GrantError;
use crate::attribute::parser::{CONTEXT_LIFETIME, IGNORED_ARGUMENT_NAME};

pub(crate) struct AttributeFn {
    attribute_identity: Path,
    is_async: bool,
    subject_type: Type,
    resource_type: Type,
    context_type: Type,
    error_type: Type,
    attribute_check_block: Block,
    subject_var: Ident,
    resource_var: Ident,
    context_var: Ident
}

impl TryFrom<(Path, ItemFn)> for AttributeFn {
    type Error = GrantError;

    fn try_from(value: (Path, ItemFn)) -> Result<Self, Self::Error> {
        let (attribute_identity, attribute_fn_def) = value;

        let attribute_check_fn = match attribute_fn_def {
            _ if attribute_fn_def.sig.variadic.is_some() => Err(GrantError::VariadicFunctionsNotSupported),
            _ if attribute_fn_def.sig.unsafety.is_some() => Err(GrantError::UnsafeFunctionsNotSupported),
            _ if attribute_fn_def.sig.abi.is_some() => Err(GrantError::ExternFunctionsNotSupported),
            _ if attribute_fn_def.sig.constness.is_some() => Err(GrantError::ConstFunctionsNotSupported),
            _ => Ok(attribute_fn_def),
        }?;

        let inner = match attribute_check_fn.clone().sig.output {
            ReturnType::Type(_, inner) => Ok(inner),
            _ => Err(GrantError::FunctionReturnTypeRequired)
        }?;

        let mut segments = match *inner {
            Type::Path(TypePath { path, .. }) => { Ok(path.segments) }
            _ => Err(GrantError::IncorrectFunctionReturnType)
        }?;

        let first_segment = segments.pop().ok_or(GrantError::IncorrectFunctionReturnType)?;

        let grant_result_ident: Ident = parse_quote!(AttributeResult);

        let error_type = match first_segment.value() {
            path_segment if path_segment.ident != grant_result_ident => Err(GrantError::ReturnTypeMustBeAttributeResult),
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

        let mut inputs = attribute_check_fn.sig.inputs.clone().into_iter();

        let (subject_var, subject_type) = match inputs.next() {
            Some(input) => match extract_type(input, true) {
                Err(GrantError::TypeMustBeAReference) => Err(GrantError::SubjectTypeMustBeAReference),
                Err(GrantError::IncorrectType) => Err(GrantError::IncorrectSubjectType),
                extract => extract
            },
            None => Err(GrantError::IncorrectNumberOfInputArguments)
        }?;

        let (resource_var, resource_type) = match inputs.next() {
            Some(input) => match extract_type(input, true) {
                Err(GrantError::TypeMustBeAReference) => Err(GrantError::ResourceTypeMustBeAReference),
                Err(GrantError::IncorrectType) => Err(GrantError::IncorrectResourceType),
                extract => extract
            },
            None => {
                let ty: Type = parse_quote! { () };
                let default_resource_var = Ident::new(IGNORED_ARGUMENT_NAME, Span::call_site());
                Ok((default_resource_var, ty))
            }
        }?;

        let (context_var, context_type) = match inputs.next() {
            Some(input) => match extract_type(input, false) {
                Err(GrantError::IncorrectType) => Err(GrantError::IncorrectContextType),
                // lifetime substitution so consumers don't need to add their own 'ctx to references
                Ok((ident, ty)) => Ok((ident, substitute_lifetime_with_context_lifetime(ty))),
                extract => extract
            },
            None => {
                let ty: Type = parse_quote! { () };
                let default_resource_var = Ident::new(IGNORED_ARGUMENT_NAME, Span::call_site());
                Ok((default_resource_var, ty))
            }
        }?;

        let attribute_check_block = *attribute_check_fn.block;
        let is_async = attribute_check_fn.sig.asyncness.is_some();

        Ok(Self {
            attribute_identity,
            is_async,
            subject_type,
            resource_type,
            context_type,
            error_type,
            attribute_check_block,
            subject_var,
            resource_var,
            context_var
        })
    }
}

impl ToTokens for AttributeFn {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let AttributeFn {
            attribute_identity,
            subject_type,
            resource_type,
            context_type,
            error_type,
            attribute_check_block,
            subject_var,
            resource_var,
            context_var,
            ..
        } = &self;


        tokens.extend(quote!{
           impl dacquiri::prelude::BaseAttribute for #attribute_identity<#subject_type, #resource_type> {
                type Subject = #subject_type;
                type Resource = #resource_type;
                type Context<'ctx> = #context_type;
                type Error = #error_type;
            }
        });

        if self.is_async {
            tokens.extend(quote!{
                #[async_trait::async_trait]
                impl dacquiri::prelude::AsyncAttribute for #attribute_identity<#subject_type, #resource_type> {
                    async fn test_async<'ctx>(#subject_var: &Self::Subject, #resource_var: &Self::Resource, #context_var: Self::Context<'ctx>) -> dacquiri::prelude::AttributeResult<Self::Error> #attribute_check_block
                }
            });
        } else {
            tokens.extend(quote!{
                impl dacquiri::prelude::SyncAttribute for #attribute_identity<#subject_type, #resource_type> {
                    fn test<'ctx>(#subject_var: &Self::Subject, #resource_var: &Self::Resource, #context_var: Self::Context<'ctx>) -> dacquiri::prelude::AttributeResult<Self::Error> #attribute_check_block
                }
            });
        }
    }
}

fn extract_type(input: FnArg, force_reference: bool) -> Result<(Ident, Type), GrantError> {
    let (pat, ty) = match input {
        FnArg::Typed(PatType { pat, ty, .. }) => Ok((pat, ty)),
        _ => Err(GrantError::IncorrectNumberOfInputArguments)
    }?;

    if force_reference {
        extract_reference_type(*pat, *ty)
    } else {
        extract_any_type(*pat, *ty)
    }
}


fn extract_reference_type(pat: Pat, ty: Type) -> Result<(Ident, Type), GrantError> {
    match (pat, ty) {
        (Pat::Wild(_), Type::Reference(inner)) => {
            let ident: Ident = Ident::new(IGNORED_ARGUMENT_NAME, Span::call_site());

            Ok((ident, *inner.elem))
        },
        (Pat::Ident(var_name), Type::Reference(inner)) => Ok((var_name.ident, *inner.elem)),
        (_, Type::Path(_) | Type::Tuple(_)) => Err(GrantError::TypeMustBeAReference),
        _ => Err(GrantError::IncorrectType)
    }
}

fn extract_any_type(pat: Pat, ty: Type) -> Result<(Ident, Type), GrantError> {
    match (pat, ty) {
        (Pat::Wild(_), ty) => {
            let ident: Ident = Ident::new(IGNORED_ARGUMENT_NAME, Span::call_site());

            Ok((ident, ty))
        },
        (Pat::Ident(var_name), ty) => Ok((var_name.ident, ty)),
        _ => Err(GrantError::IncorrectType)
    }
}

/// Recursively dives through types to replace lifetimes with 'ctx to make them work w/ context transparently
fn substitute_lifetime_with_context_lifetime(ty: Type) -> Type {
    match ty {
        Type::Reference(mut ref_type) => {
            ref_type.elem = Box::new(substitute_lifetime_with_context_lifetime(*ref_type.elem));
            ref_type.lifetime = Some(Lifetime::new(CONTEXT_LIFETIME, Span::call_site()));

            Type::Reference(ref_type)
        },
        Type::Tuple(mut tuple_type) => {
            let adjusted_types = tuple_type.elems
                .clone()
                .into_iter()
                .map(substitute_lifetime_with_context_lifetime)
                .collect::<Vec<Type>>();

            tuple_type.elems.clear();
            tuple_type.elems.extend(adjusted_types);

            Type::Tuple(tuple_type)
        },
        Type::Array(mut array_type) => {
            array_type.elem = Box::new(substitute_lifetime_with_context_lifetime(*array_type.elem));

            Type::Array(array_type)
        },
        plain => plain
    }
}
