#![deny(warnings)]
#![allow(incomplete_features)]
#![feature(generic_associated_types)]
#![feature(adt_const_params)]
#![feature(generic_arg_infer)]
#![feature(rustc_attrs)]
#![feature(marker_trait_attr)]

use dacquiri::prelude::*;
use policy::*;


fn main() -> AttributeResult<()> {
    let user = User::new(true, true);

    user.into_entity::<"user">()
        .prove::<Enabled<_, _>, "user">()?
        .print_message();

    Ok(())
}

mod policy {
    use dacquiri::prelude::*;

    pub struct User {
        enabled: bool,
        verified: bool
    }

    impl User {
        pub fn new(enabled: bool, verified: bool) -> Self {
            Self { enabled, verified }
        }
    }

    #[attribute(Enabled)]
    mod enabled {
        use super::User;

        #[attribute]
        pub fn check_enabled(user: &User) -> AttributeResult<()> {
            if user.enabled {
                Ok(())
            } else {
                Err(())
            }
        }
    }

    #[attribute(Verified)]
    mod verified {
        use super::User;

        #[attribute]
        pub fn check_verified(user: &User) -> AttributeResult<()> {
            if user.verified {
                Ok(())
            } else {
                Err(())
            }
        }
    }

    #[policy(
        entities = (
            user: User
        ),
        guard = (
            user is Enabled,
            user is Verified,
        )
    )]
    pub trait VerifiedUserPolicy {
        fn print_message(&self) {
            println!("Success!");
        }
    }
}
