use proc_macro2::{TokenStream, Ident, Span};
use quote::{ToTokens, quote};
use syn::{AttributeArgs, ItemFn, NestedMeta, Meta, Path, ItemMod, Item, Attribute, AttrStyle};
use syn::__private::TokenStream2;
use super::parser::attribute_fn::AttributeFn;

const ATTRIBUTE_FN_ATTRIBUTE: &str = "attribute";

#[derive(Debug)]
pub enum GrantError {
    MissingGrantName,
    VariadicFunctionsNotSupported,
    UnsafeFunctionsNotSupported,
    ExternFunctionsNotSupported,
    ConstFunctionsNotSupported,
    FunctionReturnTypeRequired,
    IncorrectFunctionReturnType,
    ReturnTypeMustBeAttributeResult,
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
    AttributeHasNoDefinitions,
}

pub struct AttributeBuilder {
    module_name: Ident,
    attribute_name: Path,
    attribute_fns: Vec<AttributeFn>,
    other_items: Vec<Item>,
}

impl TryFrom<(AttributeArgs, ItemMod)> for AttributeBuilder {
    type Error = GrantError;

    fn try_from((mut args, attribute_mod): (AttributeArgs, ItemMod)) -> Result<Self, Self::Error> {
        let module_name = attribute_mod.ident;

        // #[attribute(AccountEnabled)] => AccountEnabled
        let attribute_name = match args.pop() {
            Some(NestedMeta::Meta(Meta::Path(bound))) => bound,
            _ => return Err(GrantError::MissingGrantName)
        };

        let items = &attribute_mod.content.ok_or(GrantError::AttributeHasNoDefinitions)?.1;

        let mut attribute_fns = Vec::new();
        let mut other_items = Vec::new();

        for item in items {
            match item {
                // if this is an annotated attribute fn
                Item::Fn(item_fn) if is_attribute_fn(item_fn) => {
                    let attr_fn = AttributeFn::try_from((attribute_name.clone(), item_fn.clone()))?;

                    attribute_fns.push(attr_fn);
                },
                other => other_items.push(other.clone()),
            };
        }

        Ok(AttributeBuilder {
            module_name,
            attribute_name,
            attribute_fns,
            other_items
        })
    }
}

/**
    Validates that a given ItemFn is a proper attribute fn definition.

    This validates that the function is in the form

    #[attribute]
    fn some_name(...) {

    }
*/
fn is_attribute_fn(item_fn: &ItemFn) -> bool {
    item_fn.attrs
        .iter()
        // todo: add additional validation of the arguments and return type
        .any(|attribute| matches!(attribute, Attribute { style: AttrStyle::Outer, path, .. } if path.to_token_stream().to_string() == ATTRIBUTE_FN_ATTRIBUTE))
}


impl ToTokens for AttributeBuilder {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let module_identity = &self.module_name;
        let permission_identity = &self.attribute_name;
        let proving_function_name = format!("{}AttrExt", permission_identity.to_token_stream().to_string());
        let proving_function_trait = Ident::new(&proving_function_name, Span::call_site());

        tokens.extend(quote! {
            pub struct #permission_identity<S, R> {
                _subject: core::marker::PhantomData<S>,
                _resource: core::marker::PhantomData<R>,
            }
        });

        let mut attr_proving_funcs = TokenStream2::new();
        let mut mod_elems = TokenStream2::new();

        for attr_fn in &self.attribute_fns {
            mod_elems.extend(attr_fn.to_token_stream());

            attr_proving_funcs.extend(attr_fn.create_proving_function_impl());
        }

        for item in &self.other_items {
            mod_elems.extend(item.to_token_stream());
        }

        tokens.extend(quote! {
            pub use #module_identity::#proving_function_trait;

            mod #module_identity {
                use super::#permission_identity;

                #[async_trait::async_trait]
                pub trait #proving_function_trait {
                    #attr_proving_funcs
                }

                impl<T> #proving_function_trait for T
                where
                    T: dacquiri::prelude::ConstraintT + Sized {}

                #mod_elems
            }
        })
    }
}