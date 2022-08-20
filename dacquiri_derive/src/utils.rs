use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::Path;

#[derive(Clone)]
pub(crate) struct NonstandardKeyword<const KEYWORD: &'static str> {
    _keyword: Path
}

impl<const KEYWORD: &'static str> Parse for NonstandardKeyword<KEYWORD> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _keyword = match input.parse::<Path>() {
            Ok(keyword) if keyword.to_token_stream().to_string() == KEYWORD => Ok(keyword),
            _ => {
                let error_msg = format!("Expected '{}' keyword.", KEYWORD);

                Err(syn::Error::new(input.span(), error_msg))
            }
        }?;

        Ok(Self {
            _keyword
        })
    }
}
