use proc_macro2::{TokenStream, Ident, Span};
use quote::{ToTokens, quote};
use syn::{
    AttributeArgs,
    ItemFn,
    NestedMeta,
    Meta,
    Path,
    Block,
    ReturnType,
    Type,
    TypePath,
    PathArguments,
    parse_quote,
    FnArg,
    PatType,
    GenericArgument,
    Pat,
    Lifetime
};

// The name of the context lifetime.
// This should match the context lifetime defined on BaseAttribute
const CONTEXT_LIFETIME: &'static str = "'ctx";
const IGNORED_ARGUMENT_NAME: &'static str = "_";

#[derive(Debug)]
pub enum GrantError {
    MissingGrantName,
    VariadicFunctionsNotSupported,
    UnsafeFunctionsNotSupported,
    ExternFunctionsNotSupported,
    ConstFunctionsNotSupported,
    FunctionReturnTypeRequired,
    IncorrectFunctionReturnType,
    ReturnTypeMustBeGrantResult,
    GrantResultRequiresOneGeneric,
    IncorrectNumberOfInputArguments,
    IncorrectType,
    IncorrectErrorType,
    IncorrectSubjectType,
    IncorrectResourceType,
    IncorrectContextType,
    TypeMustBeAReference,
    SubjectTypeMustBeAReference,
    ResourceTypeMustBeAReference,
}

pub struct AttributeBuilder {
    is_async: bool,
    permission_name: Path,
    subject_type: Type,
    resource_type: Type,
    context_type: Type,
    error_type: Type,
    attribute_check_block: Block,
    subject_var: Ident,
    resource_var: Ident,
    context_var: Ident
}

impl TryFrom<(AttributeArgs, ItemFn)> for AttributeBuilder {
    type Error = GrantError;

    fn try_from((mut args, attribute_check_fn): (AttributeArgs, ItemFn)) -> Result<Self, Self::Error> {
        // #[grant(AccountEnabled)] => AccountEnabled
        let permission_name = match args.pop() {
            Some(NestedMeta::Meta(Meta::Path(bound))) => bound,
            _ => return Err(GrantError::MissingGrantName)
        };

        let attribute_check_fn = match attribute_check_fn {
            _ if attribute_check_fn.sig.variadic.is_some() => Err(GrantError::VariadicFunctionsNotSupported),
            _ if attribute_check_fn.sig.unsafety.is_some() => Err(GrantError::UnsafeFunctionsNotSupported),
            _ if attribute_check_fn.sig.abi.is_some() => Err(GrantError::ExternFunctionsNotSupported),
            _ if attribute_check_fn.sig.constness.is_some() => Err(GrantError::ConstFunctionsNotSupported),
            _ => Ok(attribute_check_fn),
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

        let grant_check_block = *attribute_check_fn.block;
        let is_async = attribute_check_fn.sig.asyncness.is_some();

        Ok(AttributeBuilder {
            is_async,
            permission_name,
            subject_type,
            resource_type,
            context_type,
            error_type,
            attribute_check_block: grant_check_block,
            subject_var,
            resource_var,
            context_var
        })
    }
}

// Recursively dives through types to replace lifetimes with 'ctx to make them work w/ context transparently
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
                .map(|inner_type| substitute_lifetime_with_context_lifetime(inner_type))
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

fn extract_type(input: FnArg, force_reference: bool) -> Result<(Ident, Type), GrantError> {
    let (pat, ty) = match input {
        FnArg::Typed(PatType { pat, ty, .. }) => Ok((pat, ty)),
        _ => Err(GrantError::IncorrectNumberOfInputArguments)
    }?;

    if force_reference {
        extract_reference_type(pat, ty)
    } else {
        extract_any_type(pat, ty)
    }
}

fn extract_reference_type(pat: Box<Pat>, ty: Box<Type>) -> Result<(Ident, Type), GrantError> {
    match (*pat, *ty) {
        (Pat::Wild(_), Type::Reference(inner)) => {
            let ident: Ident = Ident::new(IGNORED_ARGUMENT_NAME, Span::call_site());

            Ok((ident, *inner.elem))
        },
        (Pat::Ident(var_name), Type::Reference(inner)) => Ok((var_name.ident, *inner.elem)),
        (_, Type::Path(_) | Type::Tuple(_)) => Err(GrantError::TypeMustBeAReference),
        _ => Err(GrantError::IncorrectType)
    }
}

fn extract_any_type(pat: Box<Pat>, ty: Box<Type>) -> Result<(Ident, Type), GrantError> {
    match (*pat, *ty) {
        (Pat::Wild(_), ty) => {
            let ident: Ident = Ident::new(IGNORED_ARGUMENT_NAME, Span::call_site());

            Ok((ident, ty))
        },
        (Pat::Ident(var_name), ty) => Ok((var_name.ident, ty)),
        _ => Err(GrantError::IncorrectType)
    }
}

impl ToTokens for AttributeBuilder {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let permission_identity = &self.permission_name;

        let error_type = &self.error_type;
        
        let subject_type = &self.subject_type;
        let resource_type = &self.resource_type;
        let context_type = &self.context_type;
        
        let subject_var_name = &self.subject_var;
        let resource_var_name = &self.resource_var;
        let context_var_name = &self.context_var;

        let subject_check_impl = &self.attribute_check_block;
        
        tokens.extend(quote!{
            pub struct #permission_identity<const ID: &'static str = { dacquiri::prelude::DEFAULT_ATTRIBUTE_TAG }>(#resource_type);
        });

        tokens.extend(quote!{
           impl<const ID: &'static str> dacquiri::prelude::BaseAttribute<ID> for #permission_identity<ID> {
                type Subject = #subject_type;
                type Resource = #resource_type;
                type Context<'ctx> = #context_type;
                type Error = #error_type;

                fn new_with_resource(resource: Self::Resource) -> Self { Self(resource) }
                fn get_resource(&self) -> &Self::Resource { &self.0 }
            }
        });

        if self.is_async {
            tokens.extend(quote!{
                #[async_trait::async_trait]
                impl<const ID: &'static str> dacquiri::prelude::AsyncGrant<ID> for #permission_identity<ID> {
                    // all users can change their name
                    async fn grant_async<'ctx>(#subject_var_name: &Self::Subject, #resource_var_name: &Self::Resource, #context_var_name: Self::Context<'ctx>) -> dacquiri::prelude::AttributeResult<Self::Error> #subject_check_impl
                }
            });
        } else {
            tokens.extend(quote!{
                impl<const ID: &'static str> dacquiri::prelude::SyncGrant<ID> for #permission_identity<ID> {
                    // all users can change their name
                    fn grant<'ctx>(#subject_var_name: &Self::Subject, #resource_var_name: &Self::Resource, #context_var_name: Self::Context<'ctx>) -> dacquiri::prelude::AttributeResult<Self::Error> #subject_check_impl
                }
            });
        }
    }
}