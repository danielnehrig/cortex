use std::rc::Rc;

use crate::objects::statement::Statement;

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

impl From<Database> for Statement {
    fn from(database: Database) -> Self {
        Statement::Database(database)
    }
}

impl From<&Database> for Statement {
    fn from(database: &Database) -> Self {
        Statement::Database(database.clone())
    }
}

impl From<&Database> for Rc<str> {
    fn from(database: &Database) -> Self {
        database.name.clone()
    }
}

impl From<Database> for Rc<str> {
    fn from(database: Database) -> Self {
        database.name.clone()
    }
}
