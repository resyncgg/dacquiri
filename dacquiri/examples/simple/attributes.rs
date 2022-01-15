use std::ops::Sub;
use dacquiri::prelude::*;
use crate::error::ExampleError;
use crate::models::User;
use chrono::prelude::*;

const THIRTY_DAYS: i64 = 30;
const ADMIN_ACCOUNT_ID: u32 = 0;

#[attribute(AccountIsEnabled)]
fn check_user_is_enabled(user: &User) -> AttributeResult<ExampleError> {
    if user.is_enabled() {
        Ok(())
    } else {
        Err(ExampleError::AccountIsNotEnabled)
    }
}

#[attribute(AccountIsMatured)]
fn check_user_is_over_30_days_old(user: &User) -> AttributeResult<ExampleError> {
    let user_creation_timestamp_naive = NaiveDateTime::from_timestamp(user.get_account_creation() as i64, 0);
    let user_creation_timestamp = DateTime::from_utc(user_creation_timestamp_naive, Utc);
    let date_diff = Utc::now().sub(user_creation_timestamp);

    if date_diff.num_days() >= THIRTY_DAYS {
        Ok(())
    } else {
        Err(ExampleError::AccountIsNotMatured)
    }
}

#[attribute(AccountIsAdmin)]
fn check_is_user_admin(user: &User) -> AttributeResult<ExampleError> {
    if user.get_account_id() == ADMIN_ACCOUNT_ID {
        Ok(())
    } else {
        Err(ExampleError::AccountIsNotAdmin)
    }
}