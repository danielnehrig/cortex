#[derive(Debug, Clone)]
pub struct Database {
    pub name: String,
}

impl Database {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
