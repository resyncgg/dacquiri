use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{Block, FnArg, GenericArgument, ItemFn, Lifetime, parse_quote, Pat, Path, PathArguments, PatType, ReturnType, Type, TypePath, TypeTuple};
use crate::attribute::builder::GrantError;
use crate::attribute::parser::{CONTEXT_LIFETIME, IGNORED_ARGUMENT_NAME};

pub(crate) struct AttributeFn {
    attribute_identity: Path,
    attribute_fn_name: Ident,
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

impl AttributeFn {
    pub(crate) fn create_proving_function_impl(&self) -> TokenStream {
        let AttributeFn {
            attribute_identity,
            attribute_fn_name,
            subject_type,
            resource_type,
            context_type,
            error_type,
            context_var,
            ..
        } = &self;

        match (resource_type, context_type) {
            // async + subject
            _ if resource_type.is_unit_type() && context_type.is_unit_type() && self.is_async => {
                quote! {
                    async fn #attribute_fn_name<
                        'ctx,
                        const STAG: &'static str
                    >(self) -> Result<dacquiri::prelude::ConstraintChain<STAG, { dacquiri::prelude::DEFAULT_ELEMENT_TAG }, #attribute_identity<#subject_type, ()>, Self>, #error_type>
                        where
                            Self: dacquiri::prelude::AcquireAttribute
                                + dacquiri::prelude::HasEntityWithType<STAG, #subject_type>
                    {
                        self.prove_async::<#attribute_identity<_, _>, STAG>().await
                    }
                }
            },
            // async + subject + context
            _ if resource_type.is_unit_type() && self.is_async => {
                quote! {
                    async fn #attribute_fn_name<
                        'ctx,
                        const STAG: &'static str
                    >(self, #context_var: #context_type) -> Result<dacquiri::prelude::ConstraintChain<STAG, { dacquiri::prelude::DEFAULT_ELEMENT_TAG }, #attribute_identity<#subject_type, ()>, Self>, #error_type>
                        where
                            Self: dacquiri::prelude::AcquireAttributeWithContext<#context_type>
                                + dacquiri::prelude::HasEntityWithType<STAG, #subject_type>,
                            'ctx: 'async_trait,
                    {
                        self.prove_with_context_async::<#attribute_identity<_, _>, STAG>(#context_var).await
                    }
                }
            },
            // async + subject + resource
            // todo: figure out if we can restrict `#context_type` to Send
            _ if context_type.is_unit_type() && self.is_async => {
                quote! {
                    async fn #attribute_fn_name<
                        'ctx,
                        const STAG: &'static str,
                        const RTAG: &'static str,
                    >(self) -> Result<dacquiri::prelude::ConstraintChain<STAG, RTAG, #attribute_identity<#subject_type, #resource_type>, Self>, #error_type>
                        where
                            Self: dacquiri::prelude::AcquireAttributeWithResource
                                + dacquiri::prelude::HasEntityWithType<STAG, #subject_type>
                                + dacquiri::prelude::HasEntityWithType<RTAG, #resource_type>,
                            'ctx: 'async_trait,
                    {
                        self.prove_with_resource_async::<#attribute_identity<_, _>, STAG, RTAG>().await
                    }
                }
            },
            // async + subject + resource + context
            _ if self.is_async => {
                quote! {
                    async fn #attribute_fn_name<
                        'ctx,
                        const STAG: &'static str,
                        const RTAG: &'static str,
                    >(self, #context_var: #context_type) -> Result<dacquiri::prelude::ConstraintChain<STAG, RTAG, #attribute_identity<#subject_type, #resource_type>, Self>, #error_type>
                        where
                            Self: dacquiri::prelude::AcquireAttributeWithResourceAndContext<#context_type>
                                + dacquiri::prelude::HasEntityWithType<STAG, #subject_type>
                                + dacquiri::prelude::HasEntityWithType<RTAG, #resource_type>,
                            'ctx: 'async_trait
                    {
                        self.prove_with_resource_and_context_async::<#attribute_identity<_, _>, STAG, RTAG>(#context_var).await
                    }
                }
            },
            // subject
            _ if resource_type.is_unit_type() && context_type.is_unit_type() => {
                quote! {
                    fn #attribute_fn_name<
                        'ctx,
                        const STAG: &'static str
                    >(self) -> Result<dacquiri::prelude::ConstraintChain<STAG, { dacquiri::prelude::DEFAULT_ELEMENT_TAG }, #attribute_identity<#subject_type, ()>, Self>, #error_type>
                        where
                            Self: dacquiri::prelude::AcquireAttribute
                                + dacquiri::prelude::HasEntityWithType<STAG, #subject_type>
                    {
                        self.prove::<#attribute_identity<_, _>, STAG>()
                    }
                }
            },
            // subject + context
            _ if resource_type.is_unit_type() => {
                quote! {
                    fn #attribute_fn_name<
                        'ctx,
                        const STAG: &'static str,
                    >(self, #context_var: #context_type) -> Result<dacquiri::prelude::ConstraintChain<STAG, { dacquiri::prelude::DEFAULT_ELEMENT_TAG }, #attribute_identity<#subject_type, ()>, Self>, #error_type>
                        where
                            Self: dacquiri::prelude::AcquireAttributeWithContext<#context_type>
                                + dacquiri::prelude::HasEntityWithType<STAG, #subject_type>
                    {
                        self.prove_with_context::<#attribute_identity<_, _>, STAG>(#context_var)
                    }
                }
            },
            // subject + resource
            _ if context_type.is_unit_type() => {
                quote! {
                    fn #attribute_fn_name<
                        'ctx,
                        const STAG: &'static str,
                        const RTAG: &'static str,
                    >(self) -> Result<dacquiri::prelude::ConstraintChain<STAG, RTAG, #attribute_identity<#subject_type, #resource_type>, Self>, #error_type>
                        where
                            Self: dacquiri::prelude::AcquireAttributeWithResource
                                + dacquiri::prelude::HasEntityWithType<STAG, #subject_type>
                                + dacquiri::prelude::HasEntityWithType<RTAG, #resource_type>,
                    {
                        self.prove_with_resource::<#attribute_identity<_, _>, STAG, RTAG>()
                    }
                }
            },
            // subject + resource + context
            _ => {
                quote! {
                    fn #attribute_fn_name<
                        'ctx,
                        const STAG: &'static str,
                        const RTAG: &'static str,
                    >(self, #context_var: #context_type) -> Result<dacquiri::prelude::ConstraintChain<STAG, RTAG, #attribute_identity<#subject_type, #resource_type>, Self>, #error_type>
                        where
                            Self: dacquiri::prelude::AcquireAttributeWithResourceAndContext<#context_type>
                                + dacquiri::prelude::HasEntityWithType<STAG, #subject_type>
                                + dacquiri::prelude::HasEntityWithType<RTAG, #resource_type>,
                    {
                        self.prove_with_resource_and_context::<#attribute_identity<_, _>, STAG, RTAG>(#context_var)
                    }
                }
            }
        }
    }
}

trait TypeExt {
    fn is_unit_type(&self) -> bool;
}

impl TypeExt for Type {
    fn is_unit_type(&self) -> bool {
        matches!(self, Type::Tuple(TypeTuple { elems, .. }) if elems.is_empty())
    }
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
        let attribute_fn_name  = attribute_check_fn.sig.ident.clone();

        Ok(Self {
            attribute_identity,
            attribute_fn_name,
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
                #[dacquiri::async_trait::async_trait]
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
