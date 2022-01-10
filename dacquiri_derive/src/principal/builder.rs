use proc_macro2::TokenStream;
use quote::ToTokens;
use quote::quote;
use syn::ItemStruct;

pub(crate) struct PrincipalBuilder {
    item_struct: ItemStruct,
}

impl From<ItemStruct> for PrincipalBuilder {
    fn from(item_struct: ItemStruct) -> Self {
        Self { item_struct }
    }
}

impl ToTokens for PrincipalBuilder {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let struct_ident = &self.item_struct.ident;
        let generics = &self.item_struct.generics;

        let principal_derive = quote! {
            impl #generics dacquiri::prelude::PrincipalT for #struct_ident #generics {
                fn into_principal(self) -> Self { self }
                fn get_principal(&self) -> &Self { self }
                fn get_principal_mut(&mut self) -> &mut Self { self }
            }
        };

        tokens.extend(principal_derive);
    }
}