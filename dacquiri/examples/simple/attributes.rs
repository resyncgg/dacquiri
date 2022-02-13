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