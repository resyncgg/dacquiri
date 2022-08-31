use proc_macro::TokenStream;
use quote::ToTokens;
use crate::attribute::builder::AttributeBuilder;
use syn::{AttributeArgs, ItemFn, ItemMod, parse_macro_input};

mod builder;
pub(crate) mod parser;

/**
    #[attribute(Enabled)]
    mod enabled {
        fn check_user_enabled(user: &User) -> AttributeResult<Error> {
            if user.enabled {
                Ok(())
            } else {
                Err(Error::UserNotEnabled)
            }
        }

        fn check_session_enabled(session: &Session) -> AttributeResult<Error> {
            if session.enabled {
                Ok(())
            } else {
                Err(Error::SessionNotEnabled)
            }
        }
    }
*/

pub(crate) fn handle_attribute(args: TokenStream, input: TokenStream) -> TokenStream {
    let attribute_args = parse_macro_input!(args as AttributeArgs);
    let attribute_builder = match syn::parse::<ItemMod>(input.clone()) {
        Ok(attribute_mod) => AttributeBuilder::try_from((attribute_args, attribute_mod)),
        Err(_) => {
            let attribute_fn = syn::parse::<ItemFn>(input)
                .expect("Attributes can only annotate functions or modules.");

            AttributeBuilder::try_from((attribute_args, attribute_fn))
        }
    };

    attribute_builder
        .expect("Unable to create AttributeBuilder")
        .to_token_stream()
        .into()
}

