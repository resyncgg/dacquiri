use dacquiri::prelude::*;
use crate::attributes::Enabled;
use crate::models::User;

#[policy(
    entities = (
        user: User
    ),
    constraints = (
        user is Enabled
    )
)]
pub trait EnabledUserPolicy {
    fn print_name(&self) {
        let enabled_user: &User = self.get_entity::<_, user>();

        println!("My name is: {}", enabled_user.get_name());
    }
}