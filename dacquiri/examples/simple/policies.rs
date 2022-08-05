use dacquiri::prelude::*;
use crate::attributes::{
    Admin,
    Enabled,
    Hon
};
use crate::models::User;

#[policy(
    entities = (
        user: User,
        foo: String
    ),
    context = (
        user is Enabled,
        foo is Hon
    ),
    context = (
        user is Admin
    )
)]
pub trait EnabledUserPolicy {
    fn print_name(&self) {
        let enabled_user: &User = self.get_entity::<_, user>();

        println!("My name is: {}", enabled_user.get_name());
    }
}