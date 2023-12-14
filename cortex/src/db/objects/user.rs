use crate::objects::role::Role;

#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub name: String,
    pub password: Option<String>,
    pub roles: Vec<Role>,
    pub encrypted: bool,
}

impl User {
    pub fn new(name: &str, password: &str) -> Self {
        Self {
            name: name.to_string(),
            roles: Vec::new(),
            password: Some(password.to_string()),
            encrypted: false,
        }
    }

    pub fn add_role(&mut self, role: Role) -> &mut Self {
        self.roles.push(role);
        self
    }
}
