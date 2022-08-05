use dacquiri::prelude::*;
use crate::error::AuthorizationError;
use crate::models::User;

#[attribute(Enabled)]
pub fn check_if_user_enabled(user: &User) -> AttributeResult<AuthorizationError> {
    if user.is_enabled() {
        Ok(())
    } else {
        Err(AuthorizationError::UserNotEnabled)
    }
}

#[attribute(Admin)]
pub fn check_if_user_is_admin(user: &User) -> AttributeResult<AuthorizationError> {
    if user.get_name() == "admin" {
        Ok(())
    } else {
        Err(AuthorizationError::UserIsNotAdmin)
    }
}

#[attribute(Hon)]
pub fn check_if_string_is_hon(message: &String) -> AttributeResult<AuthorizationError> {
    if message.to_lowercase() == "hon" {
        Ok(())
    } else {
        Err(AuthorizationError::MessageIsNotHon)
    }
}