use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Database {
    pub name: Rc<str>,
}

impl Database {
    pub fn new(name: &str) -> Self {
        Self {
            name: Rc::from(name),
        }
    }
}
