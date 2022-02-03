pub struct User {
    name: String,
    user_id: u64,
    enabled: bool,
    verified: bool
}

pub struct Team {
    name: String,
    enabled: bool,
    owner: u64
}

impl User {
    pub fn new(name: impl Into<String>, user_id: u64) -> Self {
        Self {
            name: name.into(),
            user_id,
            enabled: true,
            verified: true
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

impl Team {
    pub fn new(name: impl Into<String>, owner: &User) -> Self {
        Self {
            name: name.into(),
            enabled: true,
            owner: owner.user_id
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}
