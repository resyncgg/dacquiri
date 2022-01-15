use dacquiri::prelude::Subject;

#[derive(Subject)]
pub struct User {
    username: String,
    user_id: u32,
    enabled: bool,
    account_creation: u64,
}

impl User {
    pub fn new(username: impl Into<String>, user_id: u32) -> Self {
        Self {
            username: username.into(),
            user_id,
            enabled: false,
            account_creation: chrono::offset::Utc::now().timestamp() as u64
        }
    }

    pub fn is_enabled(&self) -> bool { self.enabled }
    pub fn get_account_id(&self) -> u32 { self.user_id }
    pub fn get_account_creation(&self) -> u64 { self.account_creation }
    pub fn get_account_name(&self) -> String {
        self.username.clone()
    }

    pub(crate) fn change_name(&mut self, new_name: String) {
        self.username = new_name;
    }

    pub(crate) fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}