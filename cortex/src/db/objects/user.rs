use crate::objects::role::Role;

#[derive(Debug, Clone)]
pub struct User {
    pub name: String,
    pub roles: Vec<Role>,
}

impl User {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            roles: Vec::new(),
        }
    }

    pub fn add_role(&mut self, role: Role) -> &mut Self {
        self.roles.push(role);
        self
    }
}
