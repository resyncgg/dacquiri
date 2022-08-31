use dacquiri::prelude::*;
use crate::{AuthorizationError, User};

#[attribute(Enabled)]
pub fn check_if_user_is_enabled(user: &User) -> AttributeResult<AuthorizationError> {
    if user.is_enabled() {
        Ok(())
    } else {
        Err(AuthorizationError::UserNotEnabled)
    }
}