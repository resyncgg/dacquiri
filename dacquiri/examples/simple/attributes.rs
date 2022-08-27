use dacquiri::prelude::*;

#[attribute(Enabled)]
mod enabled {
    use crate::error::AuthorizationError;
    use crate::models::User;

    #[attribute]
    pub fn check_if_user_enabled(user: &User) -> AttributeResult<AuthorizationError> {
        if user.is_enabled() {
            Ok(())
        } else {
            Err(AuthorizationError::UserNotEnabled)
        }
    }
}