use std::rc::Rc;

#[derive(Debug, Clone)]
/// Database related information
pub struct Database {
    pub name: Rc<str>,
}

impl Database {
    /// Create a new database with the given name.
    pub fn new(name: &str) -> Self {
        Self {
            name: Rc::from(name),
        }
    }
}
