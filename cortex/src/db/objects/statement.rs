use crate::objects::{database::Database, table::Table};

#[derive(Clone)]
/// A database action is an action that is run on the database.
pub enum DbAction {
    Create,
    Drop,
    Alter,
    Insert,
}

#[derive(Clone)]
/// A statement is a single action that is run on the database.
pub enum Statement<'a> {
    Table(&'a Table, DbAction),
    Database(&'a Database, DbAction),
}
