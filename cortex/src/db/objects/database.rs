use std::rc::Rc;

use crate::objects::statement::Statement;

#[derive(Debug, Clone)]
/// Database related information
pub struct Database {
    pub name: Rc<str>,
}

impl Database {
    /// Create a new database with the given name.
    /// # Example
    /// ```
    /// use cortex::objects::database::Database;
    /// let database = Database::new("test");
    /// ```
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

impl From<&Database> for String {
    fn from(database: &Database) -> Self {
        database.name.to_string()
    }
}

impl<'a> From<&'a Database> for &'a str {
    fn from(database: &'a Database) -> &'a str {
        &database.name
    }
}

impl From<Database> for Rc<str> {
    fn from(database: Database) -> Self {
        database.name.clone()
    }
}
