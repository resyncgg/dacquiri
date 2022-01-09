use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;
use quote::quote;

pub(crate) struct PrincipalBuilder {
    identity: Ident,
}

impl From<Ident> for PrincipalBuilder {
    fn from(identity: Ident) -> Self {
        Self { identity }
    }
}

impl ToTokens for PrincipalBuilder {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let struct_ident = &self.identity;

        let principal_derive = quote! {
            impl dacquiri::prelude::PrincipalT for #struct_ident {
                fn into_principal(self) -> Self { self }
                fn get_principal(&self) -> &Self { self }
                fn get_principal_mut(&mut self) -> &mut Self { self }
            }
        };

        tokens.extend(principal_derive);
    }
}