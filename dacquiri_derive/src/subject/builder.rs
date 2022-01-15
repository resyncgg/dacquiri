use proc_macro2::TokenStream;
use quote::ToTokens;
use quote::quote;
use syn::ItemStruct;

pub(crate) struct SubjectBuilder {
    item_struct: ItemStruct,
}

impl From<ItemStruct> for SubjectBuilder {
    fn from(item_struct: ItemStruct) -> Self {
        Self { item_struct }
    }
}

impl ToTokens for SubjectBuilder {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let struct_ident = &self.item_struct.ident;
        let generics = &self.item_struct.generics;

        let subject_derive = quote! {
            impl #generics dacquiri::prelude::SubjectT for #struct_ident #generics {
                fn into_subject(self) -> Self { self }
                fn get_subject(&self) -> &Self { self }
                fn get_subject_mut(&mut self) -> &mut Self { self }
            }
        };

        tokens.extend(subject_derive);
    }
}