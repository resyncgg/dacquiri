use dacquiri::prelude::Principal;

#[derive(Principal)]
pub struct User {
    name: String,
    enabled: bool
}

impl User {
    pub fn new(name: impl Into<String>) -> Self {
        User {
            name: name.into(),
            enabled: false
        }
    }

    pub fn enable_account(&mut self) {
        self.enabled = true;
    }

    pub fn is_enabled(&self) -> bool { self.enabled }

    pub fn get_name(&self) -> &str { &self.name }

    pub(crate) fn set_name(&mut self, new_name: impl Into<String>) {
        self.name = new_name.into();
    }
}

pub struct Team {
    name: String
}

impl Team {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into()
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}