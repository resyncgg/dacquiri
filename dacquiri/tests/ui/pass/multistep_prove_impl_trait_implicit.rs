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
    let user = User::new(true);

    let context = user
        .into_entity::<"user">()
        .check_enabled::<"user">()?;

    guarded(context)?;

    Ok(())
}

fn guarded(caller: impl EnabledUserPolicy) -> AttributeResult<()> {
    let caller = caller.check_verified::<"user">()?;

    caller.print_enabled();
    caller.print_verified();

    Ok(())
}

mod policy {
    use dacquiri::prelude::*;

    pub struct User {
        enabled: bool
    }

    impl User {
        pub fn new(enabled: bool) -> Self {
            Self { enabled }
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
        pub fn check_verified(_: &User) -> AttributeResult<()> {
            Ok(())
        }
    }

    #[policy(
        entities = (
            user: User
        ),
        guard = (
            user is Enabled
        )
    )]
    pub trait EnabledUserPolicy {
        fn print_enabled(&self) {
            println!("Enabled!");
        }
    }


    #[policy(
        entities = (
            user: User
        ),
        guard = (
            user is Enabled,
            user is Verified
        )
    )]
    pub trait VerifiedUserPolicy {
        fn print_verified(&self) {
            println!("Verified!");
        }
    }
}
