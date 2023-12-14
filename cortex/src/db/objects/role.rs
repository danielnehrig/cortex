#[derive(Debug, Clone, PartialEq)]
pub struct Role {
    pub name: String,
    pub permissions: Vec<Permission>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Permission {
    pub name: String,
    pub object: String,
    pub action: String,
}

impl Role {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            permissions: Vec::new(),
        }
    }

    pub fn add_permission(&mut self, permission: Permission) -> &mut Self {
        self.permissions.push(permission);
        self
    }
}
