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

    let context = user.into_entity::<"user">()
        .prove::<Enabled, "user">()?;

    guarded(context);

    Ok(())
}

fn guarded(context: impl EnabledUserPolicy) {
    context.print_message();
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
    pub fn check_enabled(user: &User) -> AttributeResult<()> {
        if user.enabled {
            Ok(())
        } else {
            Err(())
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
        fn print_message(&self) {
            println!("Success!");
        }
    }
}
