pub struct User {
    name: String,
    enabled: bool
}

impl User {
    pub fn new(name: impl Into<String>, enabled: bool) -> Self {
        Self {
            name: name.into(),
            enabled
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub(crate) fn get_name(&self) -> &str {
        &self.name
    }
}