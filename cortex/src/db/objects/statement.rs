use crate::objects::{database::Database, table::Table};

#[derive(Clone)]
pub enum DbAction {
    Create,
    Drop,
    Alter,
    Insert,
}

#[derive(Clone)]
pub enum Statement<'a> {
    Table(&'a Table, DbAction),
    Database(&'a Database, DbAction),
}
