use proc_macro::TokenStream;

mod principal;
mod requirement;
mod grant;

#[proc_macro_derive(Principal)]
pub fn principal_macro(input: TokenStream) -> TokenStream {
    principal::handle_principal(input)
}

#[proc_macro_attribute]
pub fn requirement(args: TokenStream, input: TokenStream) -> TokenStream {
    requirement::handle_requirement(args, input)
}

#[proc_macro_attribute]
pub fn grant(args: TokenStream, input: TokenStream) -> TokenStream {
    grant::handle_grant(args, input)
}